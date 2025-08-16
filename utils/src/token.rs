use std::{fmt::{self, Formatter}, io::ErrorKind};

use crate::KeywordHandler;

#[derive(Clone, Debug)]
pub enum Token {
	Identifier(String),
	Number(i32),
	String(String),
	Keyword(&'static str),

	Dot,
	LParan,
	RParan,
	Plus,
	Minus,
	Star,
	Slash,
	Comma,
	Equal,
	NotEqual,
	Greater,
	Less,
	GreaterEqual,
	LessEqual,
	SemiColon,
	Colon,
}

impl Token {
	pub fn to_string(&self) -> String {
		match self {
			| Token::Identifier(s) => s.clone(),
			| Token::Keyword(s) => s.to_string(),
			| Token::Number(x) => x.to_string(),
			| Token::String(s) => s.clone(),

			| _ => {
				panic!("No stringyfication for the token {:?}", self)
			}
		}
	}

	pub fn to_int(&self) -> i32 {
		match self {
			| Token::Number(x) => *x,

			| _ => {
				panic!("No conversion to an integer for the token {:?}", self)
			}
		}
	}

	pub fn is_alphabetic(c: char) -> bool {
		return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
	}
	
	pub fn is_numeric(c: char) -> bool {
		return c >= '0' && c <= '9';
	}
}

impl nom::Input for Token {
	type Item = Token;

	fn input_len(&self) -> usize { todo!() }
	fn take(&self, _: usize) -> Self { todo!() }
	fn take_from(&self, _: usize) -> Self { todo!() }
	fn take_split(&self, _: usize) -> (Self, Self) { todo!() }
	fn position<P>(&self, _: P) -> Option<usize> where P: Fn { todo!() }
	fn iter_elements(&self) -> <Self as Input>::Iter { todo!() }
	fn iter_indices(&self) -> <Self as Input>::IterIndices { todo!() }
	fn slice_index(&self, _: usize) -> Result<usize, Needed> { todo!() }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
	pub line: usize,
	pub column: usize,
}

impl Position {
	pub fn zero() -> Self {
		Self { line: 0, column: 0 }
	}
}

impl std::fmt::Display for Position {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{{Line {}, Column {}}}", self.line, self.column)
	}
}

type PositionedToken = (Token, Position);

pub fn print_error<T: std::fmt::Debug>(
	pos: Position,
	file_error: Option<ErrorKind>,
	lexer_error: Option<T>,
) {
	match file_error {
		| Some(ErrorKind::UnexpectedEof) | None => {}
		| Some(e) => {
			println!("Error while reading at {}: {}", pos, e.to_string());
		}
	}
	match lexer_error {
		| None => {}
		| Some(x) => {
			println!("Lexing error at {}: {:?}", pos, x);
		}
	}
}

impl Token {
	pub fn from_string<Kwh: KeywordHandler>(kw_handler: &Kwh, pos: Position, str: String) -> Option<PositionedToken> {
		assert!(str.len() > 0);

		// If it's an identifier
		if Token::is_alphabetic(str.chars().nth(0).unwrap()) {
			if let Some(kw) = kw_handler.is_keyword(str.as_str()) {
				return Some((Token::Keyword(kw), pos));
			}

			return Some((Token::Identifier(str), pos));
		}

		// If it's a value
		if Token::is_numeric(str.chars().nth(0).unwrap()) {
			let n = str.len();

			let has_radix = "bBoOqQdDhH".contains(str.chars().nth(n - 1).unwrap_or('\0'));
			let radixless_value = if has_radix {
				&str[0..n - 1]
			} else {
				&str[0..n]
			};

			let radix = if !has_radix {
				10
			} else {
				match str.chars().nth(n - 1).unwrap() {
					| 'b' | 'B' => 2,
					| 'q' | 'Q' | 'o' | 'O' => 8,
					| 'd' | 'D' => 10,
					| 'h' | 'H' => 16,
					| _ => 10,
				}
			};

			match i32::from_str_radix(radixless_value, radix) {
				| Err(e) => {
					print_error(pos, None, Some(e.kind()));
					None
				}
				| Ok(x) => Some((Token::Number(x), pos)),
			}
		} else {
			return None;
		}
	}
}
