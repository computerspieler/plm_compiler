use crate::ast::*;
use crate::lexer::{Lexer, Position, Token};

pub struct Parser {
	lexer: Lexer,
	last_label: Option<String>,
	encountered_error: bool
}

macro_rules! parsing_error {
	($pos: ident, $msg: tt) => { {
		dbg!("TRACE: file: {}, line: {}", file!(), line!());
		println!("Parsing error at {}: {}", $pos, $msg);
		return None;
	} };
	($start: ident, $end: ident, $msg: tt) => { {
		dbg!("TRACE: file: {}, line: {}", file!(), line!());
		println!("Parsing error between {} and {}: {}",
			$start, $end, $msg);
		return None;
	} };
	($msg: tt) => { {
		dbg!("TRACE: file: {}, line: {}", file!(), line!());
		// Is it supposed to be the end ? We'll suppose that
		// for now, but it could be otherwise
		println!("Parsing error at the end: {}", $msg);
		return None;
	} };
}

macro_rules! check_token_validity {
	($tok: expr) => {
		if ($tok).is_none() {
			parsing_error!("Missing token")
		}
	}
}

macro_rules! check_token {
	($tok: expr, $goal: pat) => {
		match ($tok) {
		Some(($goal, _)) => { }
		Some((_, pos)) => {
			parsing_error!(pos, "Expected another token type")
		}
		None => {
			parsing_error!("Missing token")
		}
		}
	};
}

macro_rules! check_token_errorless {
	($tok: expr, $goal: pat) => {
		match ($tok) {
		Some(($goal, _)) => { }
		_ => { return None; }
		}
	};
}

macro_rules! constantify {
	($input: expr) => {{
		($input).bytes()
			.map(|x| x as i32)
			.collect()
	}};
}

#[derive(Debug, Clone, PartialEq)]
enum OperationType {
	UnaryOp(UnaryOperation),
	BinaryOp(BinaryOperation)
}

impl Parser {
	pub fn new(lex: Lexer) -> Parser {
		Parser {
			lexer: lex,
			last_label: None,
			encountered_error: false
		}
	}

	fn operation_priority(op: &OperationType) -> usize {
		// These priorities come from the manual
		match op {
		OperationType::UnaryOp(UnaryOperation::ExtractAddress) => { 6 }

		OperationType::BinaryOp(BinaryOperation::Multiply) => { 5 }
		OperationType::BinaryOp(BinaryOperation::Division) => { 5 }
		OperationType::BinaryOp(BinaryOperation::Modulo) => { 5 }

		OperationType::BinaryOp(BinaryOperation::Add) => { 4 }
		OperationType::BinaryOp(BinaryOperation::AddWithCarry) => { 4 }
		OperationType::BinaryOp(BinaryOperation::Substract) => { 4 }
		OperationType::BinaryOp(BinaryOperation::SubstractWithCarry) => { 4 }
		
		OperationType::BinaryOp(BinaryOperation::Equal) => { 3 }
		OperationType::BinaryOp(BinaryOperation::NotEqual) => { 3 }
		OperationType::BinaryOp(BinaryOperation::Greater) => { 3 }
		OperationType::BinaryOp(BinaryOperation::GreaterOrEqual) => { 3 }
		OperationType::BinaryOp(BinaryOperation::Less) => { 3 }
		OperationType::BinaryOp(BinaryOperation::LessOrEqual) => { 3 }
		
		OperationType::UnaryOp(UnaryOperation::Not) => { 2 }
		
		OperationType::BinaryOp(BinaryOperation::And) => { 1 }

		OperationType::BinaryOp(BinaryOperation::Or) => { 0 }
		OperationType::BinaryOp(BinaryOperation::Xor) => { 0 }
		}
	}

	fn pop_operation(pos: &Option<Position>, output_stack: &mut Vec<Expression>, operations_stack: &mut Vec<OperationType>) -> Option<()> {
		let rhs = output_stack.pop();
		if rhs.is_none() {
			match pos {
			Some(pos) => { parsing_error!(pos, "Not enough operands") }
			None => { parsing_error!("Not enough operands") }
			}
		}
		let rhs = rhs.unwrap();

		match operations_stack.pop() {
		None => {
			match pos {
			Some(pos) => { parsing_error!(pos, "Not enough operators") }
			None => { parsing_error!("Not enough operators") }
			}
		}
		Some(OperationType::UnaryOp(op)) => {
			output_stack.push(Expression::UnaryOp(op,
				Box::new(rhs)
			));
			return Some(());
		}
		Some(OperationType::BinaryOp(op)) => {
			let lhs = output_stack.pop();
			if lhs.is_none() {
				match pos {
				Some(pos) => { parsing_error!(pos, "Not enough operands") }
				None => { parsing_error!("Not enough operands") }
				}
			}
			let lhs = lhs.unwrap();

			output_stack.push(Expression::BinaryOp(op,
				Box::new(lhs),
				Box::new(rhs)
			));
			return Some(());
		}
		}
	}

	fn parse_expression(&mut self) -> Option<Expression> {
		// This is the shutting yard algorithm
		let mut output_stack = Vec::new();
		let mut operations_stack: Vec<OperationType> = Vec::new();
		
		let mut start_pos = None;
		let mut minus_can_be_for_sign = false;

		loop {
			let tok = self.lexer.peek();
			if tok.is_none() {
				break;
			}
			let (tok, pos) = tok.unwrap();
			if start_pos.is_none() {
				start_pos = Some(pos);
			}

			macro_rules! handle_op {
				($op: expr) => {{
					minus_can_be_for_sign = true;

					self.lexer.next();
					let priority = Parser::operation_priority(&$op);

					while let Some(op) = operations_stack.last() {
						if Parser::operation_priority(op) <= priority {
							break;
						}
						if Parser::pop_operation(&start_pos, &mut output_stack, &mut operations_stack).is_none() {
							return None;
						}
					}

					operations_stack.push($op);
				}};
			}

			match tok {
			Token::LParan => {
				minus_can_be_for_sign = false;
				self.lexer.next();
				match self.parse_expression() {
				None => { return None; }
				Some(e) => {
					output_stack.push(e);
					check_token!(self.lexer.next(), Token::RParan);
				}
				}
			}

			Token::Identifier(s) => {
				minus_can_be_for_sign = false;
				self.lexer.next();
				match self.lexer.peek() {
				Some((Token::LParan, _)) => {
					match self.parse_block(true, true, Parser::parse_expression) {
					None => { return None; }
					Some(args) => {
						if args.len() == 1 {
							output_stack.push(Expression::FunctionCallOrArrayElement(
								s, Box::new(args[0].clone())
							));
						} else {
							output_stack.push(Expression::FunctionCall(
								s, args
							));
						}
					}
					}
				}
				_ => {
					output_stack.push(Expression::Identifier(s));
				}
				}
			}
			Token::Number(x) => {
				minus_can_be_for_sign = false;
				self.lexer.next();
				output_stack.push(Expression::Constant(x));
			}
			Token::String(s) => {
				minus_can_be_for_sign = false;
				self.lexer.next();
				output_stack.push(Expression::String(s));
			}

			Token::Colon => {
				self.lexer.next();
				check_token!(self.lexer.peek(), Token::Equal);
				let (_, pos) = self.lexer.next().unwrap();
				
				if output_stack.len() != 1 {
					parsing_error!(pos, "Too many operands before ':='")
				}

				match &output_stack[0] {
				Expression::FunctionCallOrArrayElement(an, idx) => {
					match self.parse_expression() {
					None => { return None; }
					Some(e) => {
						return Some(Expression::VariableAssignment(
							Variable::ArrayIndex(an.clone(), idx.clone()),
							Box::new(e)
						));
					}
					}
				}
				Expression::Identifier(vn) => {
					match self.parse_expression() {
					None => { return None; }
					Some(e) => {
						return Some(Expression::VariableAssignment(
							Variable::Variable(vn.clone()),
							Box::new(e)
						));
					}
					}
				}
				_ => parsing_error!(pos, "Invalid variable for ':='")
				}
			}

			Token::Dot => {
				handle_op!(OperationType::UnaryOp(UnaryOperation::ExtractAddress));
				match self.lexer.peek() {
				Some((Token::LParan, _)) => {
					operations_stack.pop(); // Remove the dot added by the macro
					
					match self.parse_constants() {
					None => { return None; }
					Some(vals) => {
						let mut array_vals = Vec::new();
						if self.flatten_variable_initial_value(&mut array_vals, &vals, &pos).is_none() {
							return None;
						}
						output_stack.push(Expression::AddressOfConstant(
							VariableInitialValue::Array(array_vals)
						));
					}
					}
				}
				_ => {}
				}
			}
			Token::Keyword("NOT") => { handle_op!(OperationType::UnaryOp(UnaryOperation::Not)) }

			Token::Keyword("PLUS") => { handle_op!(OperationType::BinaryOp(BinaryOperation::AddWithCarry)) }
			Token::Plus => { handle_op!(OperationType::BinaryOp(BinaryOperation::Add)); }
			Token::Keyword("MINUS") => { handle_op!(OperationType::BinaryOp(BinaryOperation::SubstractWithCarry)) }
			Token::Minus => {
				let check_for_negative = minus_can_be_for_sign;
				handle_op!(OperationType::BinaryOp(BinaryOperation::Substract));

				if !check_for_negative {
					continue;
				}

				match self.lexer.peek() {
				Some((Token::Number(x), _)) => {
					self.lexer.next();
					operations_stack.pop(); // Remove the operation added in the macro
					output_stack.push(Expression::Constant(-x));
				}
				_ => {}
				}
			}

			Token::Star => { handle_op!(OperationType::BinaryOp(BinaryOperation::Multiply)) }
			Token::Slash => { handle_op!(OperationType::BinaryOp(BinaryOperation::Division)) }
			Token::Keyword("MOD") => { handle_op!(OperationType::BinaryOp(BinaryOperation::Modulo)) }

			Token::Keyword("AND") => { handle_op!(OperationType::BinaryOp(BinaryOperation::And)) }
			Token::Keyword("OR") => { handle_op!(OperationType::BinaryOp(BinaryOperation::Or)) }
			Token::Keyword("XOR") => { handle_op!(OperationType::BinaryOp(BinaryOperation::Xor)) }
			
			Token::Equal => { handle_op!(OperationType::BinaryOp(BinaryOperation::Equal)) }
			Token::NotEqual => { handle_op!(OperationType::BinaryOp(BinaryOperation::NotEqual)) }
			Token::Greater => { handle_op!(OperationType::BinaryOp(BinaryOperation::Greater)) }
			Token::GreaterEqual => { handle_op!(OperationType::BinaryOp(BinaryOperation::GreaterOrEqual)) }
			Token::Less => { handle_op!(OperationType::BinaryOp(BinaryOperation::Less)) }
			Token::LessEqual => { handle_op!(OperationType::BinaryOp(BinaryOperation::LessOrEqual)) }

			_ => { break; }
			}
		}

		while operations_stack.len() > 0 {
			if Parser::pop_operation(&start_pos, &mut output_stack, &mut operations_stack).is_none() {
				break;
			}
		}

		if operations_stack.len() > 0 || output_stack.len() != 1 {
			return None;
		}
		
		return output_stack.pop();
	}

	fn parse_variable_identifier(&mut self) -> Option<(String, Option<String>)> {
		check_token!(self.lexer.peek(), Token::Identifier(_));
		let var_name = self.lexer.next().unwrap().0;

		if let Some((Token::Keyword("BASED"), _)) = self.lexer.peek() {
			self.lexer.next();
			check_token!(self.lexer.peek(), Token::Identifier(_));
			let var_origin = self.lexer.next().unwrap().0;
			return Some((var_name.to_string(), Some(var_origin.to_string())));
		} else {
			return Some((var_name.to_string(), None));
		}
	}

	/* In the parser, what I call a block is succession following
	   the following form:
		(r1, r2, ..., rn)
		Where r1, ..., rn are succesions of tokens following the rule
		defined by the parse function.
	 */
	fn parse_block<T>(&mut self,
		show_paranthenis_error: bool,
		authorize_empty_block: bool,
		parse: fn(&mut Self) -> Option<T>
	) -> Option<Vec<T>> {
		if show_paranthenis_error {
			check_token!(self.lexer.peek(), Token::LParan);
		} else {
			check_token_errorless!(self.lexer.peek(), Token::LParan);
		}
		self.lexer.next();

		let mut output = vec![];
		if authorize_empty_block {
			if let Some((Token::RParan, _)) = self.lexer.peek() {
				self.lexer.next();
				return Some(output);
			}
		}

		loop {
			match parse(self) {
			None => { return None; }
			Some(x) => { output.push(x); }
			}

			match self.lexer.peek() {
			Some((Token::Comma, _)) => {
				self.lexer.next();
			}
			Some((Token::RParan, _)) => {
				self.lexer.next();
				break;
			}
			Some((_, pos)) => {
				parsing_error!(pos, "Invalid token for a block")
			}
			None => {
				parsing_error!("Missing token");
			}
			}
		}

		return Some(output);
	}

	fn parse_paranthesis_less_block<T>(&mut self,
		parse: fn(&mut Self) -> Option<T>
	) -> Option<Vec<T>> {
		let mut output = vec![];
		loop {
			match parse(self) {
			None => { return None; }
			Some(x) => { output.push(x); }
			}

			match self.lexer.peek() {
			Some((Token::Comma, _)) => {
				self.lexer.next();
			}
			_ => { break; }
			}
		}

		return Some(output);
	}

	fn parse_identifier_block(&mut self, show_error: bool) -> Option<Vec<String>> {
		self.parse_block(show_error, false,
			| parser | -> Option<String> {
				match parser.lexer.peek() {
				Some((Token::Identifier(s), _)) => {
					parser.lexer.next();
					return Some(s);
				}
				Some((_, pos)) => { parsing_error!(pos, "Invalid token"); }
				None => { parsing_error!("Missing token"); }
				}
			}
		)
	}

	fn parse_statement(&mut self, label: Option<String>) -> Option<Statement> {
		let tok = self.lexer.peek();
		if tok.is_none() {
			return None;
		}

		let (tok, stmt_pos) = tok.unwrap().clone();
		match tok {
		Token::Keyword("END") => {
			self.lexer.next();
			match self.lexer.next() {
			Some((Token::SemiColon, _)) => {
				Some(Statement::EndOfStatement(None))
			}
			Some((Token::Identifier(s), _)) => {
				check_token!(self.lexer.next(), Token::SemiColon);
				Some(Statement::EndOfStatement(Some(s)))
			}
			Some((_, pos)) => {
				parsing_error!(pos, "Invalid token")
			}
			None => {
				parsing_error!("Invalid token")
			}
			}
		}
		Token::Keyword("DISABLE") => {
			self.lexer.next();
			check_token!(self.lexer.next(), Token::SemiColon);
			Some(Statement::DisableInterrupt)
		}
		Token::Keyword("ENABLE") => {
			self.lexer.next();
			check_token!(self.lexer.next(), Token::SemiColon);
			Some(Statement::EnableInterrupt)
		}
		Token::Keyword("HALT") => {
			self.lexer.next();
			check_token!(self.lexer.next(), Token::SemiColon);
			Some(Statement::Halt)
		}
		Token::Keyword("GOTO") |
		Token::Keyword("GO") => {
			self.lexer.next();
			if let Token::Keyword("GO") = tok {
				check_token!(self.lexer.next(), Token::Keyword("TO"));
			}
			
			match self.lexer.next() {
			Some((Token::Number(x), _)) => {
				check_token!(self.lexer.next(), Token::SemiColon);
				Some(Statement::GoToValue(x))
			}
			Some((Token::Identifier(x), _)) => {
				check_token!(self.lexer.next(), Token::SemiColon);
				Some(Statement::GoToIdentifier(x))
			}
			Some((_, pos)) => { parsing_error!(pos, "Invalid token for GOTO") }
			None => { parsing_error!("Invalid token for GOTO") }
			}
		}
		Token::Keyword("IF")  => {
			self.lexer.next();
			let condition = self.parse_expression();
			if condition.is_none() {
				return None;
			}
			let condition = condition.unwrap();
			check_token!(self.lexer.next(), Token::Keyword("THEN"));
			match self.parse_statement(None) {
			None => { None }
			Some(Statement::EndOfStatement(_)) => {
				// Not the good position !
				parsing_error!(stmt_pos, "End statement with a condition");
			}
			Some(then_blk) => {
				match self.lexer.peek() {
				Some((Token::Keyword("ELSE"), _)) => {
					self.lexer.next();

					match self.parse_statement(None) {
					None => { None }
					Some(else_blk) => {
						Some(Statement::IfElse(condition,
							Box::new(then_blk),
							Box::new(else_blk)
						))
					}
					}
				}
				_ => {
					Some(Statement::IfElse(condition,
						Box::new(then_blk),
						Box::new(Statement::NoOperation)
					))
				}
				}
			}
			}
		}
		Token::Keyword("DO") => {
			self.lexer.next();
			let tok = self.lexer.peek();
			check_token_validity!(tok);
			let (tok, pos) = tok.unwrap();

			match tok.clone() {
			/* Block */
			Token::SemiColon => {
				self.lexer.next();
				self.parse_statement_block(None)
			}
			/* While loop */
			Token::Keyword("WHILE") => {
				self.lexer.next();
				let condition = self.parse_expression();
				if condition.is_none() {
					return None;
				}
				let condition = condition.unwrap();

				check_token!(self.lexer.next(), Token::SemiColon);
				match self.parse_statement_block(None) {
				None => { None }
				Some(blk) => {
					Some(Statement::While(condition, Box::new(blk)))
				}
				}
			}
			/* Do case block */
			Token::Keyword("CASE") => {
				self.lexer.next();
				let condition = self.parse_expression();
				if condition.is_none() {
					return None;
				}
				let condition = condition.unwrap();

				check_token!(self.lexer.next(), Token::SemiColon);
				match self.parse_statement_block(None) {
				None => { None }
				Some(Statement::Block(blk)) => {
					Some(Statement::DoCase(condition, blk))
				}
				Some(stmt) => {
					Some(Statement::DoCase(condition, vec![stmt]))
				}
				}
			}
			/* For loop */
			Token::Identifier(var) => {
				self.lexer.next();
				check_token!(self.lexer.next(), Token::Equal);

				let origin = self.parse_expression();
				check_token!(self.lexer.next(), Token::Keyword("TO"));
				let destination = self.parse_expression();
				check_token!(self.lexer.next(), Token::SemiColon);

				match (origin, destination) {
				(Some(src), Some(dst)) => {
					match self.parse_statement_block(None) {
					Some(blk) => {
						return Some(Statement::IterativeLoop(var.clone(), src, dst, Box::new(blk)));
					}
					None => { return None; }
					}
				}
				_ => { return None; }
				}
			}

			_ => { parsing_error!(pos, "Invalid token"); }
			}
		}
		Token::Keyword("CALL") => {
			self.lexer.next();
			let function_name_token = self.lexer.next();
			check_token!(function_name_token, Token::Identifier(_));
			let function_name = function_name_token.unwrap().0.to_string();

			if let Some((Token::LParan, _)) = self.lexer.peek() {
				match self.parse_block(true, 
					true,
					Parser::parse_expression
				) {
				None => { None }
				Some(args) => {
					check_token!(self.lexer.next(), Token::SemiColon);
					Some(Statement::FunctionCall(function_name, args))
				}
				}
			} else {
				check_token!(self.lexer.next(), Token::SemiColon);
				Some(Statement::FunctionCall(function_name, vec![]))
			}
		}

		Token::Keyword("DECLARE") => {
			self.lexer.next();

			let mut names: Vec<String> = Vec::new();
			let mut types: Vec<Type> = Vec::new();
			let mut values: Vec<Option<VariableInitialValue>> = Vec::new();
			
			loop {
				let mut can_have_initial_field = true;
				// It's used to compute the number of variable added
				let variable_count_beginning = names.len();

				// Parse variables' names and base address
				// And add the missing default values
				if let Some((Token::LParan, _)) = self.lexer.peek() {
					match self.parse_block(true,
						false,
						Parser::parse_variable_identifier
					) {
					None => { return None; }
					Some(vars) => {
						for (var_name, base_address) in vars {
							names.push(var_name);
							if let Some(x) = base_address {
								can_have_initial_field = false;
								values.push(
									Some(VariableInitialValue::ValueOfPointer(x))
								);
							} else {
								values.push(None);
							}
						}
					}
					}
				} else {
					match self.parse_variable_identifier() {
					None => { return None; }
					Some((var_name, var_based_ptr)) => {
						names.push(var_name);
						if let Some(x) = var_based_ptr {
							can_have_initial_field = false;
							values.push(
								Some(VariableInitialValue::ValueOfPointer(x))
							);
						} else {
							values.push(None);
						}
					}
					}
				}

				let variable_count = names.len();
				let variables_added = variable_count - variable_count_beginning;

				// Parse the dimension
				let mut dimension = 1;
				let mut dimension_defined = false;

				if let Some((Token::LParan, _)) = self.lexer.peek() {
					self.lexer.next();
					match self.lexer.next() {
					Some((Token::Number(n), pos)) => {
						if n <= 0 {
							parsing_error!(pos, "Invalid dimension");
						}
						dimension = n as usize;
						dimension_defined = true;
					}
					_ => { return None; }
					}
					check_token!(self.lexer.next(), Token::RParan);
				}

				// Parse the type
				let var_type = self.parse_type_from_token(dimension);
				match var_type {
				None => { return None; }
				Some(Type::Data) => {
					can_have_initial_field = false;
					if dimension_defined {
						parsing_error!(stmt_pos, "Can't set a dimension when defining a variable with the type DATA");
					}
					if variables_added > 1 {
						parsing_error!(stmt_pos, "Can't declare multiples variables with the type DATA");
					}

					let mut vals = Vec::new();
					match self.parse_constants() {
					None => { return None; }
					Some(constants) => {
						if self.flatten_variable_initial_value(&mut vals, &constants, &stmt_pos).is_none() {
							return None;
						}
					}
					}

					// We know for sure that there's only
					// one variable added, so there's no problem
					if values[variable_count - 1].is_some() {
						parsing_error!(stmt_pos, "Can't set an address for a DATA variable");
					}
					values[variable_count - 1] = Some(VariableInitialValue::ReadOnlyArray(vals));
					types.push(Type::Data);
				}
				Some(Type::Macro) => {
					can_have_initial_field = false;
					match self.lexer.next() {
					Some((Token::String(content), pos)) => {
						for _ in 0 .. variables_added {
							// Make sure that no base address was
							// specified for macros
							if values.pop().unwrap().is_some() {
								parsing_error!(pos, "A macro can't have a base address");
							}
							self.lexer.add_macro(names.pop().unwrap(),
							pos, content.clone());
						}
					}
					Some((_, pos)) => { parsing_error!(pos, "Invalid value for a macro"); }
					None => { parsing_error!("Invalid value for a macro"); }
					}
				}
				Some(Type::Byte(n)) => {
					for _ in 0 .. variables_added {
						types.push(Type::Byte(n));
					}
				}
				Some(Type::Address(n)) => {
					for _ in 0 .. variables_added {
						types.push(Type::Address(n));
					}
				}
				Some(Type::Void) => { panic!("Impossible") }
				}

				// Check for the INITIAL keyword
				if let Some((Token::Keyword("INITIAL"), pos)) = self.lexer.peek() {
					if !can_have_initial_field {
						parsing_error!(pos, "Can't declare an initial value here");
					}
				
					let pos = pos.clone();
					self.lexer.next();
					match self.parse_constants() {
					None => { return None; }
					Some(constants) => {
						let constants_count = constants.len();
						if constants_count > variables_added {
							parsing_error!(pos, "Too many values for the declaration");
						}
						if constants_count < variables_added {
							parsing_error!(pos, "Not enough values for the declaration");
						}

						for i in 0 .. constants_count {
							values[i + variable_count_beginning] = Some(constants[i].clone());
						}
					}
					}
				}

				match self.lexer.peek() {	
				Some((Token::SemiColon, _)) => {
					self.lexer.next();
					break;
				}
				Some((Token::Comma, _)) => {
					self.lexer.next();
				}
				_ => { break; }
				}
			}

			return Some(Statement::VariableDeclaration(names, types, values));
		}

		// Variable assignment & Labels
		Token::LParan |
		Token::Identifier(_) => {
			let variables: Vec<Variable>;

			match self.parse_identifier_block(false) {
			Some(names) => {
				variables = names.into_iter()
					.map(|name| Variable::Variable(name))
					.collect();
			}
			None => {
				match self.parse_paranthesis_less_block(|parser| -> Option<Variable> {
					check_token!(parser.lexer.peek(), Token::Identifier(_));
					let var_name = parser.lexer.next()
						.unwrap().0
						.to_string();

					if let Some((Token::LParan, _)) = parser.lexer.peek() {
						parser.lexer.next();
						match parser.parse_expression() {
						None => { None }
						Some(idx) => {
							check_token!(parser.lexer.next(), Token::RParan);
							Some(Variable::ArrayIndex(var_name, Box::new(idx)))
						}
						}
					} else {
						Some(Variable::Variable(var_name))
					}
				}) {
				None => { return None; }
				Some(x) => { variables = x; }
				}
			}
			};
			
			match self.lexer.next() {
			// Variable assignment
			Some((Token::Equal, _)) => {
				let e = self.parse_expression();
				if e.is_none() {
					return None;
				}
				check_token!(self.lexer.next(), Token::SemiColon);
	
				let mut output_expr = e.unwrap();
				for v in variables.into_iter().rev() {
					output_expr = Expression::VariableAssignment(v,
						Box::new(output_expr));
				}
	
				return Some(Statement::Expression(output_expr));
			}

			// Label declaration
			Some((Token::Colon, pos)) => {
				if variables.len() == 1 {
					match &variables[0] {
					Variable::Variable(lbl) => {
						return Some(Statement::Label(lbl.clone()));
					}
					_ => {}
					}
				}
				parsing_error!(pos, "Invalid label name")
			}
			Some((_, pos)) => { parsing_error!(pos, "Invalid token") }
			None => { parsing_error!(stmt_pos, "Invalid token") }
			}
		}
		Token::Keyword("RETURN") => {
			self.lexer.next();
			
			return Some(Statement::Return(
				match self.lexer.peek() {
				Some((Token::SemiColon, _)) => {
					self.lexer.next();
					None
				}
				_ => {
					let e = self.parse_expression();
					check_token!(self.lexer.next(), Token::SemiColon);
					e
				}
				}
			));
		}
		Token::Keyword("PROCEDURE") => {
			self.lexer.next();

			if label.is_none() {
				parsing_error!(stmt_pos, "No name has been provided for the procedure");
			}
			let label = label.unwrap();

			let args;
			if let Some((Token::LParan, _)) = self.lexer.peek() {
				match self.parse_identifier_block(true) {
				None => { return None; }
				Some(a) => {
					args = a;
				}
				}
			} else {
				args = vec![];
			}

			let return_type;
			match self.lexer.peek() {
			Some((Token::Keyword(_), _)) => {
				match self.parse_type_from_token(1) {
				Some(t) => { return_type = t; }
				None => { return None; }
				}
			}
			Some((Token::SemiColon, _)) => {
				return_type = Type::Void;
			}
			Some((_, pos)) => { parsing_error!(pos, "Invalid token") }
			None => { parsing_error!("Invalid token") }
			}

			check_token!(self.lexer.next(), Token::SemiColon);
			
			match self.parse_statement_block(Some(label)) {
			None => { return None; }
			Some(block) => {
				return Some(Statement::Procedure(
					args, return_type,
					Box::new(block)
				));
			}
			}
		}

		// Do NOT consume this token
		// It will be reused to check whether we're at the end of the file
		Token::Keyword("EOF") => { return Some(Statement::EndOfFile); }

		Token::SemiColon => {
			self.lexer.next();
			return Some(Statement::NoOperation);
		}

		_ => { parsing_error!(stmt_pos, "Invalid token"); }
		}
	}

	fn flatten_variable_initial_value(&self, vals: &mut Vec<i32>, constants: &Vec<VariableInitialValue>, pos: &Position) -> Option<()>{
		for constant in constants {
			match constant {
			VariableInitialValue::Value(x) => { vals.push(*x) }
			VariableInitialValue::Array(arr) => {
				for x in arr {
					vals.push(*x);
				}
			}
			_ => {
				parsing_error!(pos, "Got an unknown identifier");
			}
			}
		}

		return Some(());
	}

	fn parse_type_from_token(&mut self, count: usize) -> Option<Type> {
		match self.lexer.next() {
		Some((Token::Keyword("BYTE"), _)) => { Some(Type::Byte(count)) }
		Some((Token::Keyword("ADDRESS"), _)) => { Some(Type::Address(count)) }
		Some((Token::Keyword("DATA"), _)) => {
			Some(Type::Data)
		}
		Some((Token::Keyword("LITERALLY"), _)) => {
			Some(Type::Macro)
		}
		Some((_, pos)) => { parsing_error!(pos, "Unknown type"); }
		None => { parsing_error!("Unknown type"); }
		}
	}

	fn parse_constant(&mut self) -> Option<VariableInitialValue> {
		match self.lexer.next() {
		Some((Token::Minus, _)) => {
			match self.lexer.next() {
			Some((Token::Number(x), _)) => {
				Some(VariableInitialValue::Value(-x))
			}
			Some((_, pos)) => {
				parsing_error!(pos, "Was looking for a value")
			}
			None => {
				parsing_error!("Was looking for a value")
			}
			}
		}
		Some((Token::Number(x), _)) => {
			Some(VariableInitialValue::Value(x))
		}
		Some((Token::String(s), _)) => {
			Some(VariableInitialValue::Array(constantify!(s)))
		}
		Some((_, pos)) => {
			parsing_error!(pos, "Invalid token")
		}
		None => { parsing_error!("Unfinished declaration") }
		}
	}

	fn parse_constants(&mut self) -> Option<Vec<VariableInitialValue>> {
		check_token!(self.lexer.next(), Token::LParan);
		let mut output = Vec::new();
		loop {
			match self.parse_constant() {
			None => { return None; }
			Some(val) => { output.push(val); }
			}

			match self.lexer.next() {
			Some((Token::RParan, _)) => { break; }
			Some((Token::Comma, _)) => { }
			Some((_, pos)) => { parsing_error!(pos, "Invalid token") }
			None => { parsing_error!("Unfinished declaration") }
			}
		}
		Some(output)
	}

	fn parse_statement_block(&mut self, expected_identifier: Option<String>) -> Option<Statement> {
		let mut last_label = None;
		let mut statements: Vec<Statement> = vec![];

		loop {
			match self.parse_statement(last_label) {
			Some(Statement::EndOfStatement(x)) => {
				match (x, expected_identifier) {
				(None, Some(_)) => { parsing_error!("Missing identifier") }
				(Some(_), None) => { parsing_error!("Exceeding token") }
				(None, None) => { }
				(Some(x), Some(expected_identifier)) => {
					if x != *expected_identifier {
						parsing_error!("Difference between the identifiers");
					}
				}
				}

				// Small optimization of the AST
				// It's not necessary, but it makes it
				// easier to debug
				let nb_stmts = statements.len();
				if nb_stmts == 1 {
					return Some(statements[0].clone());
				}

				return Some(Statement::Block(statements));
			}
			Some(Statement::Label(lbl)) => {
				last_label = Some(lbl.clone());
				statements.push(Statement::Label(lbl));
			}
			Some(stmt) => {
				last_label = None;
				statements.push(stmt);
			}
			None => { return None; }
			}
		}
	}

	pub fn reached_eos(&mut self) -> bool {
		!self.encountered_error &&
		(self.lexer.reached_eos() || !self.lexer.peek().is_none())
	}
}

use std::iter::Iterator;

impl Iterator for Parser {
	type Item = Statement;

	fn next(&mut self) -> Option<Statement> {
		if self.encountered_error {
			return None;
		}
		
		let lbl = self.last_label.clone();
		match self.parse_statement(lbl) {
		Some(Statement::EndOfFile) => {
			self.last_label = None;
			return None;
		}
		Some(Statement::Label(lbl)) => {
			self.last_label = Some(lbl.clone());
			return Some(Statement::Label(lbl));
		}
		Some(stmt) => {
			self.last_label = None;
			return Some(stmt);
		}
		None => {
			self.encountered_error = true;
			return None;
		}
		}
	}
}
