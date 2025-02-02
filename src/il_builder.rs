/*

fn convert_statement(stmt: ast::Statement) -> backend::ast::Statement<String> {
	use backend::ast::Statement::*;
	use backend::ast::Expression::*;
	match stmt {
	ast::Statement::Block(blk) =>
		Block(blk.into_iter().map(convert_statement).collect()),

	ast::Statement::DoCase(e, code) =>
		Switch(convert_expression(e), code.into_iter().map(convert_statement).collect()),

	ast::Statement::Return(None) => Return(None),
	ast::Statement::Return(Some(e)) =>
		Return(Some(convert_expression(e))),

	ast::Statement::FunctionCall(f, args) =>
		Expression(FunctionCall(f,
			args.into_iter().map(convert_expression).collect()
		)),

	ast::Statement::Expression(e) =>
		Expression(convert_expression(e)),

	ast::Statement::IfElse(cond, if_blk, else_blk) =>
		IfElse(convert_expression(cond),
			Box::new(convert_statement(*if_blk)),
			Box::new(convert_statement(*else_blk))
		),

	ast::Statement::DisableInterrupt => DisableInterrupt,
	ast::Statement::EnableInterrupt => EnableInterrupt,
	ast::Statement::Halt => Halt,
	ast::Statement::EndOfFile |
	ast::Statement::NoOperation => NoOperation,
/*
	While(Expression, Box<Statement>),
	IterativeLoop(String, Expression, Expression, Box<Statement>),
	VariableDeclaration(Vec<String>, Vec<Type>, Vec<Option<VariableInitialValue>>),
	EndOfStatement(Option<String>),
	Label(String, Box<Statement>),
	GoToValue(i32),
	GoToIdentifier(String),
	Procedure(Vec<String>, Type, Box<Statement>),
	ProgramBasis(i32)
*/
	}
}
 */

extern crate backend;

use std::collections::HashMap;
use std::collections::VecDeque;

use crate::ast;
use backend::ast::*;
use backend::typing::TypeCheckable;

type VariableIdx = usize;
pub struct Environment {
	vars_type: Vec<Option<Type>>,
	vars_names: HashMap<String, VariableIdx>,
}

impl TypeCheckable<Environment> for VariableIdx {
	fn get_type(&self, env: &Environment) -> Option<Type> {
		if *self >= env.vars_type.len() {
			return None;
		}
		return env.vars_type[*self].clone();
	}
}

pub struct BackendConverter<InputType: Iterator<Item = ast::Statement> + EOSDetector> {
	input: InputType,
	statement_queue: VecDeque<Statement<VariableIdx>>,
	env: Environment,
}

impl<InputType: Iterator<Item = ast::Statement> + EOSDetector> BackendConverter<InputType> {
	pub fn new(input: InputType) -> Self {
		Self {
			input: input,
			statement_queue: VecDeque::new(),
			env: Environment {
				vars_type: Vec::new(),
				vars_names: HashMap::new(),
			},
		}
	}

	pub fn get_environment(self) -> Environment {
		self.env
	}

	fn convert_binary_operator(op: ast::BinaryOperation) -> backend::ast::BinaryOperation {
		use backend::ast::BinaryOperation::*;
		match op {
			| ast::BinaryOperation::Add => Add,
			| ast::BinaryOperation::AddWithCarry => AddWithCarry,
			| ast::BinaryOperation::Substract => Substract,
			| ast::BinaryOperation::SubstractWithCarry => SubstractWithCarry,

			| ast::BinaryOperation::Multiply => Multiply,
			| ast::BinaryOperation::Division => Division,
			| ast::BinaryOperation::Modulo => Modulo,

			| ast::BinaryOperation::And => And,
			| ast::BinaryOperation::Or => Or,
			| ast::BinaryOperation::Xor => Xor,

			| ast::BinaryOperation::Equal => Equal,
			| ast::BinaryOperation::NotEqual => NotEqual,
			| ast::BinaryOperation::Greater => Greater,
			| ast::BinaryOperation::GreaterOrEqual => GreaterOrEqual,
			| ast::BinaryOperation::Less => Less,
			| ast::BinaryOperation::LessOrEqual => LessOrEqual,
		}
	}

	fn convert_unary_operator(op: ast::UnaryOperation) -> backend::ast::UnaryOperation {
		use backend::ast::UnaryOperation::*;
		match op {
			| ast::UnaryOperation::Not => Not,
			| ast::UnaryOperation::ExtractAddress => Reference,
		}
	}

	fn convert_expression(
		&self,
		e: ast::Expression,
	) -> Option<backend::ast::Expression<VariableIdx>> {
		use backend::ast::Expression::*;
		match e {
			| ast::Expression::Constant(x) => {
				Some(Constant(backend::ast::Constant::Value(x, Type::Number)))
			}

			| ast::Expression::Identifier(id) => match self.env.vars_names.get(&id) {
				| None => None,
				| Some(idx) => Some(Variable(*idx)),
			},

			| ast::Expression::BinaryOp(op, lhs, rhs) => {
				let lhs = self.convert_expression(*lhs);
				let rhs = self.convert_expression(*rhs);

				match (lhs, rhs) {
					| (Some(lhs), Some(rhs)) => Some(BinaryOp(
						Self::convert_binary_operator(op),
						Box::new(lhs),
						Box::new(rhs),
					)),
					| _ => None,
				}
			}

			| ast::Expression::UnaryOp(op, e) => match self.convert_expression(*e) {
				| Some(lhs) => Some(UnaryOp(Self::convert_unary_operator(op), Box::new(lhs))),
				| None => None,
			},

			| ast::Expression::FunctionCall(f, args) => {
				let mut converted_args = Vec::with_capacity(args.len());
				let mut args_conversion_iter = args.into_iter().map(|e| self.convert_expression(e));

				while let Some(arg) = args_conversion_iter.next() {
					match arg {
						| Some(arg) => {
							converted_args.push(arg);
						}
						| None => {
							return None;
						}
					}
				}

				Some(FunctionCall(f, converted_args))
			}

			| ast::Expression::FunctionCallOrArrayElement(_, _) => panic!("Shouldn't be here"),

			| _ => panic!("TODO"),
		}
	}

	fn parse_statement(&mut self, stmt: ast::Statement) -> Result<(), ()> {
		//self.statement_stack.push_back(Statement::NoOperation);
		//Err(())
		match stmt {
			| ast::Statement::Block(blk) => {
				for stmt in blk.into_iter() {
					match self.parse_statement(stmt) {
						| Ok(()) => {}
						| Err(()) => {
							return Err(());
						}
					}
				}

				return Ok(());
			}
			| _ => {
				return Err(());
			}
		}
	}
}

impl<InputType: Iterator<Item = ast::Statement> + EOSDetector> Iterator
	for BackendConverter<InputType>
{
	type Item = Statement<VariableIdx>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if self.statement_queue.len() > 0 {
				return self.statement_queue.pop_front();
			}

			/* Convert the next statement */
			let stmt = self.input.next();
			if stmt.is_none() {
				return None;
			}

			match self.parse_statement(stmt.unwrap()) {
				| Ok(_) => {}
				| Err(_) => {
					return None;
				}
			}
		}
	}
}

use crate::traits::EOSDetector;
impl<InputType: Iterator<Item = ast::Statement> + EOSDetector> EOSDetector
	for BackendConverter<InputType>
{
	fn reached_eos(&mut self) -> bool {
		self.input.reached_eos() && self.statement_queue.len() == 0
	}
}
