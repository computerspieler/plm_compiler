use plm::lexer::*;
use plm::parser::*;
use plm::ast::*;

macro_rules! compare_ast {
	($input: literal, $goal: expr) => {{
		let mut parser = Parser::new(Lexer::from_string(String::from($input)));
		match ($goal) as Option<Vec<Statement>> {
		None => {
			// Consumes the iterator
			while let Some(_) = parser.next() {}
			assert!(!parser.reached_eos());
		}
		Some(x) => {
			for stmt in x {
				match parser.next() {
				None => { assert!(false, "Ended too early"); }
				Some(p) => { assert_eq!(p, stmt); }
				}
			}
		}
		}
	}};
}

macro_rules! constantify {
	($input: expr) => {{
		($input).bytes()
			.map(|x| x as i32)
			.collect()
	}};
}

#[test]
fn test_expression0() {
	compare_ast!("IF 80h THEN DISABLE;", Some(vec![
		Statement::IfElse(Expression::Constant(0x80), 
			Box::new(Statement::DisableInterrupt),
			Box::new(Statement::NoOperation)
		)
	]))
}

#[test]
fn test_expression1() {
	compare_ast!("IF (80h) THEN DISABLE;", Some(vec![
		Statement::IfElse(Expression::Constant(0x80), 
			Box::new(Statement::DisableInterrupt),
			Box::new(Statement::NoOperation)
		)
	]))
}

#[test]
fn test_expression2() {
	compare_ast!("IF arr THEN DISABLE;", Some(vec![
		Statement::IfElse(
			Expression::Identifier("ARR".to_string()),
			Box::new(Statement::DisableInterrupt),
			Box::new(Statement::NoOperation)
		)
	]))
}

#[test]
fn test_expression3() {
	compare_ast!("IF f() THEN DISABLE;", Some(vec![
		Statement::IfElse(
			Expression::FunctionCall(
				"F".to_string(),
				vec![]
			), 
			Box::new(Statement::DisableInterrupt),
			Box::new(Statement::NoOperation)
		)
	]))
}

#[test]
fn test_expression4() {
	compare_ast!("IF arr(80h) THEN DISABLE;", Some(vec![
		Statement::IfElse(
			Expression::FunctionCallOrArrayElement(
				"ARR".to_string(),
				Box::new(Expression::Constant(0x80))
			), 
			Box::new(Statement::DisableInterrupt),
			Box::new(Statement::NoOperation)
		)
	]))
}

#[test]
fn test_expression5() {
	compare_ast!("IF arr(.'HELLO') THEN DISABLE;", Some(vec![
		Statement::IfElse(
			Expression::FunctionCallOrArrayElement(
				"ARR".to_string(),
				Box::new(Expression::UnaryOp(UnaryOperation::ExtractAddress,
					Box::new(Expression::String("HELLO".to_string()))
				))
			), 
			Box::new(Statement::DisableInterrupt),
			Box::new(Statement::NoOperation)
		)
	]))
}

#[test]
fn test_invalid_statement0() {
	compare_ast!("WIWI;", None)
}

#[test]
fn test_invalid_statement1() {
	compare_ast!("0123;", None)
}

#[test]
fn test_invalid_statement2() {
	compare_ast!(";", None)
}

#[test]
fn test_invalid_statement3() {
	compare_ast!(";;;;;", None)
}

#[test]
fn test_invalid_token_in_expression0() {
	compare_ast!("IF arr(80hh) THEN DISABLE;", None)
}

#[test]
fn test_invalid_function_call0() {
	compare_ast!("f(80h);", None)
}

#[test]
fn test_invalid_function_call1() {
	compare_ast!("CALL f1(80h, Hello, 8r+8);", None)
}

#[test]
fn test_invalid_function_call2() {
	compare_ast!("CALL f1(80h, Hello, 8+8)", None)
}

#[test]
fn test_valid_function_call0() {
	compare_ast!("CALL f1(80h);", Some(vec![
		Statement::FunctionCall("F1".to_string(), vec![
			Expression::Constant(0x80)
		])
	]))
}

#[test]
fn test_valid_function_call1() {
	compare_ast!("CALL f1();", Some(vec![
		Statement::FunctionCall("F1".to_string(), vec![])
	]))
}

#[test]
fn test_valid_function_call2() {
	compare_ast!("CALL f1(80h, Hello, 8+8);", Some(vec![
		Statement::FunctionCall("F1".to_string(), vec![
			Expression::Constant(0x80),
			Expression::Identifier("HELLO".to_string()),
			Expression::BinaryOp(BinaryOperation::Add,
				Box::new(Expression::Constant(8)),
				Box::new(Expression::Constant(8))
			)
		])
	]))
}

#[test]
fn test_valid_function_call3() {
	compare_ast!("CALL f1;", Some(vec![
		Statement::FunctionCall("F1".to_string(), vec![])
	]))
}

#[test]
fn test_valid_statement0() {
	compare_ast!("", Some(vec![]))
}

#[test]
fn test_valid_variable_declaration0() {
	compare_ast!("DECLARE x BYTE;", Some(vec![
		Statement::VariableDeclaration(
			vec!["X".to_string()],
			vec![Type::Byte(1)],
			vec![None]
		)
	]))
}

#[test]
fn test_valid_variable_declaration1() {
	compare_ast!("DECLARE (x1, x2, y) BYTE;", Some(vec![
		Statement::VariableDeclaration(
			vec![
				"X1".to_string(),
				"X2".to_string(),
				"Y".to_string()
			],
			vec![
				Type::Byte(1),
				Type::Byte(1),
				Type::Byte(1)
			],
			vec![None, None, None]
		)
	]))
}

#[test]
fn test_valid_variable_declaration2() {
	compare_ast!("DECLARE x1 BYTE, x2 BYTE, y BYTE;", Some(vec![
		Statement::VariableDeclaration(
			vec![
				"X1".to_string(),
				"X2".to_string(),
				"Y".to_string()
			],
			vec![
				Type::Byte(1),
				Type::Byte(1),
				Type::Byte(1)
			],
			vec![None, None, None]
		)
	]))
}

#[test]
fn test_valid_variable_declaration3() {
	compare_ast!("DECLARE arr(10) BYTE;", Some(vec![
		Statement::VariableDeclaration(
			vec!["ARR".to_string()],
			vec![Type::Byte(10)],
			vec![None]
		)
	]))
}

#[test]
fn test_valid_variable_declaration4() {
	compare_ast!("DECLARE x1 BYTE, x2 ADDRESS, y BYTE;", Some(vec![
		Statement::VariableDeclaration(
			vec![
				"X1".to_string(),
				"X2".to_string(),
				"Y".to_string()
			],
			vec![
				Type::Byte(1),
				Type::Address(1),
				Type::Byte(1)
			],
			vec![None, None, None]
		)
	]))
}

#[test]
fn test_valid_variable_declaration5() {
	compare_ast!("DECLARE arr$address(8) ADDRESS;", Some(vec![
		Statement::VariableDeclaration(
			vec!["ARRADDRESS".to_string()],
			vec![Type::Address(8)],
			vec![None]
		)
	]))
}

#[test]
fn test_valid_variable_declaration6() {
	compare_ast!("DECLARE x BYTE INITIAL(10);", Some(vec![
		Statement::VariableDeclaration(
			vec!["X".to_string()],
			vec![Type::Byte(1)],
			vec![
				Some(VariableInitialValue::Value(10))
			]
		)
	]))
}

#[test]
fn test_valid_variable_declaration7() {
	compare_ast!("DECLARE Y ADDRESS INITIAL(32);", Some(vec![
		Statement::VariableDeclaration(
			vec!["Y".to_string()],
			vec![Type::Address(1)],
			vec![
				Some(VariableInitialValue::Value(32))
			]
		)
	]))
}

#[test]
fn test_valid_variable_declaration8() {
	compare_ast!("DECLARE (X1, X2, X3) BYTE INITIAL(32, 10, 8);", Some(vec![
		Statement::VariableDeclaration(
			vec![
				"X1".to_string(),
				"X2".to_string(),
				"X3".to_string()
			],
			vec![
				Type::Byte(1),
				Type::Byte(1),
				Type::Byte(1)
			],
			vec![
				Some(VariableInitialValue::Value(32)),
				Some(VariableInitialValue::Value(10)),
				Some(VariableInitialValue::Value(8))
			]
		)
	]))
}

#[test]
fn test_valid_variable_declaration9() {
	let string_in_bytes: Vec<i32> =  constantify!("Hello, world.");
	compare_ast!("DECLARE GREETINGS DATA ('Hello, world.');", Some(vec![
		Statement::VariableDeclaration(
			vec!["GREETINGS".to_string()],
			vec![Type::Data],
			vec![Some(VariableInitialValue::ReadOnlyArray(string_in_bytes))]
		)
	]))
}

#[test]
fn test_valid_variable_declaration10() {
	compare_ast!("DECLARE GREETINGS DATA (10, 11);", Some(vec![
		Statement::VariableDeclaration(
			vec!["GREETINGS".to_string()],
			vec![Type::Data],
			vec![Some(VariableInitialValue::ReadOnlyArray(vec![10, 11]))]
		)
	]))
}

#[test]
fn test_valid_variable_declaration11() {
	// From CP/M 1.4 CCP's source code
	let mut data = vec![7,3];
	data.append(&mut constantify!("DIRECT"));
	data.push(0xFF);
	data.push(6);
	data.push(3);
	data.append(&mut constantify!("ERASE"));
	data.push(0xFF);
	data.push(5);
	data.push(2);
	data.append(&mut constantify!("TYPE"));
	data.push(0xFF);
	data.push(5);
	data.push(3);
	data.append(&mut constantify!("SAVE"));
	data.push(0xFF);
	data.push(3);
	data.push(2);
	data.append(&mut constantify!("A:"));
	data.push(0xFF);
	data.push(3);
	data.push(2);
	data.append(&mut constantify!("B:"));
	data.push(0xFF);
	data.push(7);
	data.push(2);
	data.append(&mut constantify!("ASSIGN"));
	data.push(0xFF);
	data.push(7);
	data.push(3);
	data.append(&mut constantify!("RENAME"));
	data.push(0xFF);
	data.push(0);

	compare_ast!("DECLARE INTVEC DATA (7,3,'DIRECT',0FFH,6,3,'ERASE',0FFH,5,2,'TYPE',0FFH,5,3,'SAVE',0FFH,3,2,'A:',0FFH,3,2,'B:',0FFH,7,2,'ASSIGN',0FFH,7,3,'RENAME',0FFH,0);", Some(vec![
		Statement::VariableDeclaration(
			vec!["INTVEC".to_string()],
			vec![Type::Data],
			vec![Some(VariableInitialValue::ReadOnlyArray(data))]
		)
	]))
}

#[test]
fn test_valid_variable_declaration12() {
	compare_ast!("DECLARE A$PTR ADDRESS, A BASED A$PTR BYTE;", Some(vec![
		Statement::VariableDeclaration(
			vec![
				"APTR".to_string(),
				"A".to_string()
			],
			vec![
				Type::Address(1),
				Type::Byte(1)
			],
			vec![
				None,
				Some(VariableInitialValue::ValueOfPointer("APTR".to_string()))
			]
		)
	]))
}

#[test]
fn test_valid_variable_declaration13() {
	compare_ast!("DECLARE NUMBER ADDRESS, (BASE,CHARS,ZERO$SUPPRESS,I,J) BYTE;", Some(vec![
		Statement::VariableDeclaration(
			vec![
				"NUMBER".to_string(),
				"BASE".to_string(),
				"CHARS".to_string(),
				"ZEROSUPPRESS".to_string(),
				"I".to_string(),
				"J".to_string()
			],
			vec![
				Type::Address(1),
				Type::Byte(1),
				Type::Byte(1),
				Type::Byte(1),
				Type::Byte(1),
				Type::Byte(1)
			],
			vec![None, None, None, None, None, None]
		)
	]))
}

#[test]
fn test_valid_variable_declaration14() {
	compare_ast!("DECLARE GREETINGS BASED GREETINGS$PTR ADDRESS;", Some(vec![
		Statement::VariableDeclaration(
			vec!["GREETINGS".to_string()],
			vec![Type::Address(1)],
			vec![
				Some(VariableInitialValue::ValueOfPointer("GREETINGSPTR".to_string()))
			]
		)
	]))
}

#[test]
fn test_valid_variable_declaration15() {
	//TODO
	compare_ast!("DECLARE BUFFA ADDRESS INITIAL(80H), (BUFF BASED BUFFA) (128) BYTE;", Some(vec![
		Statement::VariableDeclaration(
			vec![
				"BUFFA".to_string(),
				"BUFF".to_string()
			],
			vec![
				Type::Address(1),
				Type::Byte(128)
			],
			vec![
				Some(VariableInitialValue::Value(0x80)),
				Some(VariableInitialValue::ValueOfPointer("BUFFA".to_string()))
			]
		)
	]))
}

#[test]
fn test_valid_variable_declaration16() {
	//TODO
	compare_ast!("DECLARE BUFFA ADDRESS INITIAL(80H), BUFF(128) BYTE;", Some(vec![
		Statement::VariableDeclaration(
			vec![
				"BUFFA".to_string(),
				"BUFF".to_string()
			],
			vec![
				Type::Address(1),
				Type::Byte(128)
			],
			vec![
				Some(VariableInitialValue::Value(0x80)),
				None
			]
		)
	]))
}

#[test]
fn test_invalid_variable_declaration0() {
	compare_ast!("DECLARE x BOTE;", None)
}

#[test]
fn test_invalid_variable_declaration1() {
	compare_ast!("DECLARE (x1, x2, ) BYTE;", None)
}

#[test]
fn test_invalid_variable_declaration2() {
	compare_ast!("DECLARE x1 BYTE, x2 BYTE, y;", None)
}

#[test]
fn test_invalid_variable_declaration3() {
	compare_ast!("DECLARE arr(xxx) BYTE;", None)
}

#[test]
fn test_invalid_variable_declaration4() {
	compare_ast!("DECLARE 0x1 BYTE, x2 ADDRESS, y BYTE;", None)
}

#[test]
fn test_invalid_variable_declaration5() {
	compare_ast!("DECLARE arr address(8) ADDRESS;", None)
}

#[test]
fn test_invalid_variable_declaration6() {
	compare_ast!("DECLARE x BYTE INITIAL();", None)
}

#[test]
fn test_invalid_variable_declaration7() {
	compare_ast!("DECLARE ADDRESS INITIAL(32, 32);", None)
}

#[test]
fn test_invalid_variable_declaration8() {
	compare_ast!("DECLARE (X1, ADDRESS, X3) BYTE INITIAL(32, 10, 8);", None)
}

#[test]
fn test_invalid_variable_declaration9() {
	compare_ast!("DECLARE GREETINGS DATA ();", None)
}

#[test]
fn test_invalid_variable_declaration10() {
	compare_ast!("DECLARE GREETINGS(5) DATA ('Hello');", None)
}


#[test]
fn test_invalid_variable_declaration11() {
	compare_ast!("DECLARE GREETINGS(5) DATA;", None)
}

#[test]
fn test_invalid_variable_declaration12() {
	compare_ast!("DECLARE GREETINGS DATA, X BYTE;", None)
}

#[test]
fn test_invalid_variable_declaration13() {
	compare_ast!("DECLARE GREETINGS BYTE (5) INITIAL ('Hello');", None)
}

#[test]
fn test_invalid_variable_declaration14() {
	compare_ast!("DECLARE GREETINGS(5) BASED GREETINGS$PTR BYTE INITIAL ('Hello');", None)
}

#[test]
fn test_invalid_variable_declaration15() {
	compare_ast!("DECLARE GREETINGS BASED GREETINGS$PTR (5) BYTE INITIAL ('Hello');", None)
}

#[test]
fn test_invalid_variable_declaration16() {
	compare_ast!("DECLARE GREETINGS BASED GREETINGS$PTR DATA;", None)
}

#[test]
fn test_invalid_variable_declaration17() {
	compare_ast!("DECLARE GREETINGS BASED GREETINGS$PTR DATA (10);", None)
}

#[test]
fn test_valid_macro_declaration0() {
	compare_ast!("DECLARE FOREVER LITERALLY 'WHILE TRUE';", Some(vec![
		Statement::VariableDeclaration(vec![], vec![], vec![])
	]))
}

#[test]
fn test_valid_macro_declaration1() {
	compare_ast!("DECLARE FOREVER LITERALLY 'WHILE TRUE'; DO FOREVER; END;", Some(vec![
		Statement::VariableDeclaration(vec![],vec![], vec![]),
		Statement::While(Expression::Identifier("TRUE".to_string()),
			Box::new(Statement::Block(vec![]))
		)
	]))
}

#[test]
fn test_valid_macro_declaration2() {
	compare_ast!("DECLARE CR LITERALLY '0DH', LF LITERALLY '0AH';\nDECLARE MSG DATA (CR, LF);", Some(vec![
		Statement::VariableDeclaration(vec![], vec![], vec![]),
		Statement::VariableDeclaration(
			vec!["MSG".to_string()],
			vec![Type::Data],
			vec![
				Some(VariableInitialValue::ReadOnlyArray(vec![0x0D, 0x0A]))
			]
		)
	]))
}

#[test]
fn test_valid_macro_declaration3() {
	compare_ast!("DECLARE CR LITERALLY '0DH', LF LITERALLY '0AH', CRLF LITERALLY 'CR,LF';\n\
		DECLARE MSG DATA (CRLF);",
	Some(vec![
		Statement::VariableDeclaration(vec![], vec![], vec![]),
		Statement::VariableDeclaration(
			vec!["MSG".to_string()],
			vec![Type::Data],
			vec![
				Some(VariableInitialValue::ReadOnlyArray(vec![0x0D, 0x0A]))
			]
		)
	]))
}

#[test]
fn test_valid_variable_assignment0() {
	compare_ast!("$Q = 0;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::Constant(0))
		))
	]))
}

#[test]
fn test_valid_variable_assignment1() {
	compare_ast!("($Q1, Q2, X) = 0;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q1".to_string()), 
			Box::new(Expression::VariableAssignment(
				Variable::Variable("Q2".to_string()),
				Box::new(Expression::VariableAssignment(
					Variable::Variable("X".to_string()), 
					Box::new(Expression::Constant(0))
				))
			))
		))
	]))
}

#[test]
fn test_valid_variable_assignment2() {
	compare_ast!("($Q1, Q2, X) = $Q1+1+2;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q1".to_string()), 
			Box::new(Expression::VariableAssignment(
				Variable::Variable("Q2".to_string()),
				Box::new(Expression::VariableAssignment(
					Variable::Variable("X".to_string()), 
					Box::new(Expression::BinaryOp(
						BinaryOperation::Add,
						Box::new(Expression::Identifier("Q1".to_string())),
						Box::new(Expression::BinaryOp(
							BinaryOperation::Add,
							Box::new(Expression::Constant(1)),
							Box::new(Expression::Constant(2))
						))
					))
				))
			))
		))
	]))
}

#[test]
fn test_valid_variable_assignment3() {
	compare_ast!("arr(2) = 0;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::ArrayIndex("ARR".to_string(),
				Box::new(Expression::Constant(2))
			), 
			Box::new(Expression::Constant(0))
		))
	]))
}

#[test]
fn test_valid_variable_assignment4() {
	compare_ast!("$Q1, Q2, X = $Q1+1+2;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q1".to_string()), 
			Box::new(Expression::VariableAssignment(
				Variable::Variable("Q2".to_string()),
				Box::new(Expression::VariableAssignment(
					Variable::Variable("X".to_string()), 
					Box::new(Expression::BinaryOp(
						BinaryOperation::Add,
						Box::new(Expression::Identifier("Q1".to_string())),
						Box::new(Expression::BinaryOp(
							BinaryOperation::Add,
							Box::new(Expression::Constant(1)),
							Box::new(Expression::Constant(2))
						))
					))
				))
			))
		))
	]))
}

#[test]
fn test_valid_variable_assignment5() {
	compare_ast!("arr(2), Q2, X = $Q1+1+2;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::ArrayIndex("ARR".to_string(),
				Box::new(Expression::Constant(2))
			), 
			Box::new(Expression::VariableAssignment(
				Variable::Variable("Q2".to_string()),
				Box::new(Expression::VariableAssignment(
					Variable::Variable("X".to_string()), 
					Box::new(Expression::BinaryOp(
						BinaryOperation::Add,
						Box::new(Expression::Identifier("Q1".to_string())),
						Box::new(Expression::BinaryOp(
							BinaryOperation::Add,
							Box::new(Expression::Constant(1)),
							Box::new(Expression::Constant(2))
						))
					))
				))
			))
		))
	]))
}

#[test]
fn test_invalid_variable_assignment0() {
	compare_ast!("arr(2 = 0;", None)
}

#[test]
fn test_invalid_variable_assignment1() {
	compare_ast!("arr(2) := 0;", None)
}

#[test]
fn test_invalid_variable_assignment2() {
	compare_ast!("arr(2), d) := 0;", None)
}

#[test]
fn test_invalid_variable_assignment3() {
	compare_ast!("arr(2, d := 0;", None)
}

#[test]
fn test_valid_variable_reference0() {
	compare_ast!("$Q = .0;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::UnaryOp(
				UnaryOperation::ExtractAddress,
				Box::new(Expression::Constant(0))
			))
		))
	]))
}

#[test]
fn test_valid_variable_reference1() {
	compare_ast!("$Q = .X;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::UnaryOp(
				UnaryOperation::ExtractAddress,
				Box::new(Expression::Identifier("X".to_string()))
			))
		))
	]))
}

#[test]
fn test_valid_variable_reference2() {
	compare_ast!("$Q = .(0);", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::AddressOfConstant(
				VariableInitialValue::Array(vec![0])
			))
		))
	]))
}

#[test]
fn test_valid_variable_reference3() {
	let mut data = vec![0];
	data.append(&mut constantify!("Message"));

	compare_ast!("$Q = .(0, 'Message');", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::AddressOfConstant(
				VariableInitialValue::Array(data)
			))
		))
	]))
}

#[test]
fn test_valid_operation0() {
	compare_ast!("$Q = 1+2;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::BinaryOp(
				BinaryOperation::Add,
				Box::new(Expression::Constant(1)),
				Box::new(Expression::Constant(2))
			))
		))
	]))
}

#[test]
fn test_valid_operation1() {
	compare_ast!("$Q = 1+2*3;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::BinaryOp(
				BinaryOperation::Add,
				Box::new(Expression::Constant(1)),
				Box::new(Expression::BinaryOp(
					BinaryOperation::Multiply,
					Box::new(Expression::Constant(2)),
					Box::new(Expression::Constant(3))
				))
			))
		))
	]))
}

#[test]
fn test_valid_operation2() {
	compare_ast!("$Q = 2*3+1;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::BinaryOp(
				BinaryOperation::Add,
				Box::new(Expression::BinaryOp(
					BinaryOperation::Multiply,
					Box::new(Expression::Constant(2)),
					Box::new(Expression::Constant(3))
				)),
				Box::new(Expression::Constant(1))
			))
		))
	]))
}

#[test]
fn test_valid_operation3() {
	compare_ast!("$Q = 2*3+1 MOD 6;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::BinaryOp(
				BinaryOperation::Add,
				Box::new(Expression::BinaryOp(
					BinaryOperation::Multiply,
					Box::new(Expression::Constant(2)),
					Box::new(Expression::Constant(3))
				)),
				Box::new(Expression::BinaryOp(
					BinaryOperation::Modulo,
					Box::new(Expression::Constant(1)),
					Box::new(Expression::Constant(6))
				)),
			))
		))
	]))
}

#[test]
fn test_valid_operation4() {
	compare_ast!("$Q = 6 MOD 2*3+1;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::BinaryOp(
				BinaryOperation::Add,
				Box::new(Expression::BinaryOp(
					BinaryOperation::Modulo,
					Box::new(Expression::Constant(6)),
					Box::new(Expression::BinaryOp(
						BinaryOperation::Multiply,
						Box::new(Expression::Constant(2)),
						Box::new(Expression::Constant(3))
					)),
				)),
				Box::new(Expression::Constant(1))
			))
		))
	]))
}

#[test]
fn test_valid_operation5() {
	compare_ast!("$Q = 1 + -2;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::BinaryOp(
				BinaryOperation::Add,
				Box::new(Expression::Constant(1)),
				Box::new(Expression::Constant(-2))
			))
		))
	]))
}

#[test]
fn test_valid_operation6() {
	compare_ast!("$Q = NOT 1+4 AND 2 OR 3;", Some(vec![
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::BinaryOp(
				BinaryOperation::Or,
				Box::new(Expression::BinaryOp(
					BinaryOperation::And,
					Box::new(Expression::UnaryOp(
						UnaryOperation::Not,
						Box::new(Expression::BinaryOp(
							BinaryOperation::Add,
							Box::new(Expression::Constant(1)),
							Box::new(Expression::Constant(4))
						))
					)),
					Box::new(Expression::Constant(2))
				)),
				Box::new(Expression::Constant(3))
			))
		))
	]))
}

#[test]
fn test_valid_label0() {
	compare_ast!("lbl: $Q = 1;", Some(vec![
		Statement::Label("LBL".to_string(), Box::new(
			Statement::Expression(Expression::VariableAssignment(
				Variable::Variable("Q".to_string()), 
				Box::new(Expression::Constant(1))
			))
		))
	]))
}

#[test]
fn test_valid_label1() {
	compare_ast!("3200h:", Some(vec![
		Statement::ProgramBasis(0x3200)
	]))
}

#[test]
fn test_valid_label2() {
	compare_ast!("3200h: $Q = 1;", Some(vec![
		Statement::ProgramBasis(0x3200),
		Statement::Expression(Expression::VariableAssignment(
			Variable::Variable("Q".to_string()), 
			Box::new(Expression::Constant(1))
		))
	]))
}

#[test]
fn test_invalid_label0() {
	compare_ast!("15lbl: $Q = 1;", None)
}

#[test]
fn test_invalid_label1() {
	compare_ast!("lbl, lbl: $Q = 1;", None)
}

#[test]
fn test_invalid_label2() {
	compare_ast!("lbl lbl: $Q = 1;", None)
}

#[test]
fn test_valid_procedure0() {
	compare_ast!("f: PROCEDURE; END f;", Some(vec![
		Statement::Label("F".to_string(),
			Box::new(Statement::Procedure(
				vec![],
				Type::Void,
				Box::new(Statement::Block(vec![]))
			))
		)
	]))
}

#[test]
fn test_valid_procedure1() {
	compare_ast!("f: PROCEDURE(x, y); END f;", Some(vec![
		Statement::Label("F".to_string(),
			Box::new(Statement::Procedure(
				vec!["X".to_string(), "Y".to_string()],
				Type::Void,
				Box::new(Statement::Block(vec![]))
			))
		)
	]))
}

#[test]
fn test_valid_procedure2() {
	compare_ast!("f: PROCEDURE(x, y) BYTE; END f;", Some(vec![
		Statement::Label("F".to_string(),
			Box::new(Statement::Procedure(
				vec!["X".to_string(), "Y".to_string()],
				Type::Byte(1),
				Box::new(Statement::Block(vec![]))
			))
		)
	]))
}

#[test]
fn test_valid_procedure3() {
	compare_ast!("f: PROCEDURE BYTE; END f;", Some(vec![
		Statement::Label("F".to_string(),
			Box::new(Statement::Procedure(
				vec![],
				Type::Byte(1),
				Box::new(Statement::Block(vec![]))
			))
		)
	]))
}

#[test]
fn test_valid_return0() {
	compare_ast!("RETURN;", Some(vec![
		Statement::Return(None)
	]))
}

#[test]
fn test_valid_return1() {
	compare_ast!("RETURN(2);", Some(vec![
		Statement::Return(Some(
			Expression::Constant(2)
		))
	]))
}

#[test]
fn test_valid_return2() {
	compare_ast!("DO; RETURN; END;", Some(vec![
		Statement::Return(None)
	]))
}

#[test]
fn test_valid_do_case0() {
	compare_ast!("DO CASE i; RETURN; END;", Some(vec![
		Statement::DoCase(
			Expression::Identifier("I".to_string()),
			vec![
				Statement::Return(None)
			]
		)
	]))
}

#[test]
fn test_valid_do_case1() {
	compare_ast!("DO CASE i; RETURN(0); RETURN(1); RETURN(3); END;", Some(vec![
		Statement::DoCase(
			Expression::Identifier("I".to_string()),
			vec![
				Statement::Return(Some(Expression::Constant(0))),
				Statement::Return(Some(Expression::Constant(1))),
				Statement::Return(Some(Expression::Constant(3)))
			]
		)
	]))
}

#[test]
fn test_valid_do_case2() {
	compare_ast!("DO CASE (i+1); RETURN; END;", Some(vec![
		Statement::DoCase(
			Expression::BinaryOp(BinaryOperation::Add,
				Box::new(Expression::Identifier("I".to_string())),
				Box::new(Expression::Constant(1))
			),
			vec![
				Statement::Return(None)
			]
		)
	]))
}

#[test]
fn test_valid_do_case3() {
	compare_ast!("DO CASE I; END;", Some(vec![
		Statement::DoCase(
			Expression::Identifier("I".to_string()),
			vec![]
		)
	]))
}

#[test]
fn test_valid_no_op0() {
	compare_ast!(";", Some(vec![
		Statement::NoOperation
	]))
}

#[test]
fn test_valid_no_op1() {
	compare_ast!(";;;", Some(vec![
		Statement::NoOperation,
		Statement::NoOperation,
		Statement::NoOperation
	]))
}

#[test]
fn test_valid_no_op2() {
	compare_ast!("IF 1 THEN ; ELSE ;", Some(vec![
		Statement::IfElse(Expression::Constant(1),
			Box::new(Statement::NoOperation),
			Box::new(Statement::NoOperation)
		)
	]))
}

#[test]
fn test_invalid_if0() {
	compare_ast!("IF THEN ; ELSE ;", None)
}

#[test]
fn test_invalid_if1() {
	compare_ast!("IF 1 THAN ; ELSE ;", None)
}
