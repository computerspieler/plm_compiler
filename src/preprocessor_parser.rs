use crate::parser_macros::*;
use crate::lexer::*;

#[derive(Debug)]
pub struct CompilerArguments {
	program_basis: u32
}

struct CompilerArgument {
	name: char,
	accepted_range: Option<(i32, i32)>,
	_help: &'static str,
	callback: fn(&mut CompilerArguments, i32)
}


const ARGUMENTS_COUNT: usize = 10;
static ARGUMENTS: [CompilerArgument; ARGUMENTS_COUNT] = [
	/* PASS 1 Compiler Controls */
	CompilerArgument{
		name: 'L',
		accepted_range: Some((1, 79)),
		_help:
"Leftmargin.
Specifies the first character position processed on each input line.
All leading characters are ignored.",
		callback: |_output: &mut CompilerArguments, _val: i32| { /* TODO */ }
	},
	CompilerArgument{
		name: 'P',
		accepted_range: Some((0, 1)),
		_help: "Echo input if 1, suppress echo if 0.",
		callback: |_output: &mut CompilerArguments, _val: i32| { /* TODO */ }
	},
	CompilerArgument{
		name: 'R',
		accepted_range: Some((1, 80)),
		_help: "Rightmargin, ignore trailing characters on each input record.",
		callback: |_output: &mut CompilerArguments, _val: i32| { /* TODO */ }
	},
	CompilerArgument{
		name: 'W',
		accepted_range: Some((1, 120)),
		_help: "Maximum number of characters per output line.",
		callback: |_output: &mut CompilerArguments, _val: i32| { /* TODO */ }
	},

	/* PASS 2 Compiler Controls */
	CompilerArgument{
		name: 'F',
		accepted_range: Some((0, 1)),
		_help: "Display decoded memory initialization.",
		callback: |_output: &mut CompilerArguments, _val: i32| { /* TODO */ }
	},
	CompilerArgument{
		name: 'T',
		accepted_range: Some((0, 1)),
		_help:
"Display cross-reference table of approximate memory address
versus source line number.",
		callback: |_output: &mut CompilerArguments, _val: i32| { /* TODO */ }
	},
	CompilerArgument{
		name: 'M',
		accepted_range: Some((0, 1)),
		_help: "Display symbol table.",
		callback: |_output: &mut CompilerArguments, _val: i32| { /* TODO */ }
	},
	CompilerArgument{
		name: 'Q',
		accepted_range: Some((0, 1)),
		_help:
"If 1 then object file is written in BNPF,
otherwise the object file is written in Hex format.",
		callback: |_output: &mut CompilerArguments, _val: i32| { /* TODO */ }
	},
	CompilerArgument{
		name: 'H',
		accepted_range: None,
		_help:
"Header.
Decimal address at which generated code should start.
I.e., the start of the program's ISA.",
		callback: |_output: &mut CompilerArguments, _| { /* TODO */ }
	},
	CompilerArgument{
		name: 'V',
		accepted_range: None,
		_help:
"Page number of first page of the VSA.
I.e., variable storage, stack, etc.
If set to zero the first availabe page above the ISA is used.",
		callback: |_output: &mut CompilerArguments, _| { /* TODO */ }
	},
];

pub fn parse_compiler_arguments(lex: &mut Lexer) -> Option<CompilerArguments> {
	let mut output = CompilerArguments {
		program_basis: 0
	};

	loop {
		let tok = lex.next();
		if tok.is_none() {
			if lex.reached_eos() {
				break;
			}
			return None;
		}

		let (tok, pos) = tok.unwrap();
		match tok {
		Token::Identifier(flag) => {
			let flag = flag.chars().nth(0).unwrap();
			
			let mut found = false;
			for arg in ARGUMENTS.iter() {
				if arg.name != flag {
					continue;
				}

				match arg.accepted_range {
				None => {
					(arg.callback)(&mut output, 0);
				}
				Some((start_range, end_range)) => {
					check_token!(lex.next(), Token::Equal);
					let val = lex.next();
					check_token!(val, Token::Number(_));
					let (val, pos) = val.unwrap();
					let val = val.to_int();
	
					if start_range > val || end_range < val {
						parsing_error!(pos, "Invalid value");
					}

					(arg.callback)(&mut output, val);
				}
				}
				found = true;
				break;
			}

			if !found {
				return None;
			}
		}
		Token::Number(x) => {
			check_token!(lex.next(), Token::Colon);
			output.program_basis = x as u32;
			break;
		}
		_ => { parsing_error!(pos, "Invalid token"); }
		}
	}

	return Some(output);
}