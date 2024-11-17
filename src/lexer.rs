#[derive(Clone, Debug)]
pub enum Token {
	Identifier(String),
	Number(i32),
	String(String),
	Keyword(&'static str),
	
	Dot,
	LParan, RParan,
	Plus, Minus,
	Star, Slash,
	Comma,
	Equal, NotEqual,
	Greater, Less,
	GreaterEqual, LessEqual,
	SemiColon, Colon
}

impl Token {
	pub fn to_string(&self) -> String {
		match self {
		Token::Identifier(s) => { s.clone() }
		Token::Keyword(s) => { s.to_string() }
		Token::Number(x) => { x.to_string() }
		Token::String(s) => { s.clone() }
		
		_ => { panic!("No stringyfication for the token {:?}", self) }
		}
	}
}

const KEYWORDS_COUNT: usize = 34;
static KEYWORDS: [&'static str; KEYWORDS_COUNT] = [
	"ADDRESS",
	"AND",
	"BASED",
	"BY",
	"BYTE",
	"CALL",
	"CASE",
	"DATA",
	"DECLARE",
	"DISABLE",
	"DO",
	"ELSE",
	"ENABLE",
	"END",
	"EOF",
	"GO",
	"GOTO",
	"HALT",
	"IF",
	"INITIAL",
	"INTERRUPT",
	"LABEL",
	"LITERALLY",
	"MINUS",
	"MOD",
	"NOT",
	"OR",
	"PLUS",
	"PROCEDURE",
	"RETURN",
	"THEN",
	"TO",
	"WHILE",
	"XOR",
];

#[derive(Clone, Copy, Debug)]
pub struct Position {
	line: usize,
	column: usize
}

impl Position {
	pub fn zero() -> Self {
		Self {
			line: 0,
			column: 0
		}
	}
}

impl std::fmt::Display for Position {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{{Line {}, Column {}}}", self.line, self.column)
	}
}

fn print_error<T: std::fmt::Debug>(pos: Position, file_error: Option<ErrorKind>, lexer_error: Option<T>) {
	match file_error {
	Some(ErrorKind::UnexpectedEof) |
	None => {}
	Some(e) => {
		println!("Error while reading at {}: {}", pos, e.to_string());
	}
	}
	match lexer_error {
	None => {}
	Some(x) => {
		println!("Lexing error at {}: {:?}", pos, x);
	}
	}
}

impl Token {
	fn from_string(pos: Position, str: String) -> Option<(Token, Position)> {
		assert!(str.len() > 0);

		// If it's an identifier
		if Token::is_alphabetic(str.chars().nth(0).unwrap()) {
			for i in 0 .. KEYWORDS_COUNT {
				if KEYWORDS[i] == str {
					return Some((Token::Keyword(KEYWORDS[i]), pos));
				}
			}

			return Some((Token::Identifier(str), pos));
		}

		// If it's a value
		if Token::is_numeric(str.chars().nth(0).unwrap()) {
			let n = str.len();
			
			let has_radix =
				"bBoOqQdDhH".contains(str.chars().nth(n-1).unwrap_or('\0'));
			let radixless_value =
				if has_radix { &str[0 .. n-1] }
				else { &str[0 .. n] };

			let radix =
				if !has_radix { 10 }
				else {
					match str.chars().nth(n-1).unwrap() {
					'b' | 'B' => { 2 }
					'q' | 'Q' |
					'o' | 'O' => { 8 }
					'd' | 'D' => { 10 }
					'h' | 'H' => { 16 }
					_ => { 10 }
					}
				};

			match i32::from_str_radix(radixless_value, radix) {
			Err(e)  => {
				print_error(pos, None, Some(e.kind()));
				None
			}
			Ok(x) => { Some((Token::Number(x), pos)) }
			}
		} 
		else {
			return None;
		}
	}

	fn is_alphabetic(c: char) -> bool {
		return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
	}
	fn is_numeric(c: char) -> bool {
		return c >= '0' && c <= '9';
	}
}

use core::fmt;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::File;

pub struct Lexer {
	input_file: Option<File>,
	input_string: Option<(Vec<u8>, usize)>,
	stash: Option<(char, Position)>,
	cursor_position: Position,

	macros_idx: HashMap<String, usize>,
	macros: Vec<Vec<(Token, Position)>>,
	running_macro: Option<(usize, usize)>,

	peeked_token: Option<Option<(Token, Position)>>
}

impl Lexer {
	pub fn from_file(f: File) -> Self {
		Self {
			input_file: Some(f),
			input_string: None,
			stash: None,
			cursor_position: Position {
				line: 1,
				column: 0
			},

			macros_idx: HashMap::new(),
			macros: Vec::new(),
			running_macro: None,

			peeked_token: None
		}
	}

	pub fn from_string(str: String) -> Self {
		Self {
			input_file: None,
			input_string: Some((str.into_bytes(), 0)),
			stash: None,
			cursor_position: Position {
				line: 1,
				column: 0
			},
			macros_idx: HashMap::new(),
			macros: Vec::new(),
			running_macro: None,

			peeked_token: None
		}
	}

	fn next_byte(&mut self, buf: &mut [u8; 1]) -> Result<usize, Error> {
		if let Some(input_file) = self.input_file.as_mut() {
			return input_file.read(buf);
		} else if let Some((input_str, pos)) = self.input_string.as_mut() {
			if *pos >= input_str.len() {
				return Err(Error::from(ErrorKind::UnexpectedEof));
			}
			buf[0] = input_str[*pos];
			*pos += 1;
			return Ok(1); 
		} else {
			return Err(Error::from(ErrorKind::InvalidInput));
		}
	}

	fn next_character(&mut self, format: bool) -> (Result<char, Error>, Position) {
		if let Some((c, pos)) = self.stash {
			self.stash = None;
			return (Ok(c), pos);
		}

		let mut buf = [0 as u8; 1];
		
		match self.next_byte(&mut buf) {
		Err(e) => { return (Err(e), self.cursor_position); }
		Ok(n) => {
			if n == 0 {
				return (Err(Error::from(ErrorKind::UnexpectedEof)), self.cursor_position);
			}
			self.cursor_position.column += 1;

			// Check if they are unicode
			if buf[0] & 0x80 != 0 {
				// If a byte starts with 10xxxxxx, it means
				// that it's a byte for a character's encoding.
				// It can be ignored
				if buf[0] & 0xC0 == 0x80 {
					self.cursor_position.column -= 1;
					return self.next_character(format);
				} else {
					// And ignore them, since they should be
					// treated as whitespace
					return (Ok(' '), self.cursor_position);
				}
			}

			let c = buf[0] as char;

			if c == '\n' {
				self.cursor_position.line += 1;
				self.cursor_position.column = 0;
			}

			if format {
				return (Ok(c.to_ascii_uppercase()), self.cursor_position);
			} else {
				return (Ok(c), self.cursor_position);
			}
		}
		}
	}

	fn copy_macros(&mut self, input: &Self) {
		if self.macros.len() == 0 {
			self.macros = input.macros.clone();
			self.macros_idx = input.macros_idx.clone();
		} else {
			for (keyword, idx) in input.macros_idx.iter() {
				self.macros_idx.insert(keyword.clone(), self.macros.len());
				self.macros.push(input.macros[*idx].clone());
			}
		}
	}

	pub fn add_macro(&mut self, keyword: String, initial_position: Position, data: String) {
		let mut lex = Lexer::from_string(data);
		lex.copy_macros(&self);
		lex.cursor_position = initial_position;
		self.macros_idx.insert(keyword, self.macros.len());
		self.macros.push(lex.collect());
	}

	fn launch_macro(&mut self, keyword: &String) -> bool {
		match self.macros_idx.get(keyword) {
		None => { return false; }
		Some(x) => {
			self.running_macro = Some((*x, 0));
			return true;
		}
		}
	}

	pub fn peek(&mut self) -> Option<(Token, Position)> {
		if self.peeked_token.is_none() {
			self.peeked_token = Some(self.next());
		}
		return self.peeked_token.clone().unwrap();
	}

	pub fn reached_eos(&mut self) -> bool {
		match self.peek() {
		Some(_) => { false }
		None => {
			let mut buf = [0 as u8; 1];
		
			match self.next_byte(&mut buf) {
			Err(e) => {
				e.kind() == ErrorKind::UnexpectedEof
			}
			_ => { false }
			}
		}
		}
	}
}

use std::io::{Error, ErrorKind, Read};
use std::iter::Iterator;

impl Iterator for Lexer {
	type Item = (Token, Position);

	fn next(&mut self) -> Option<(Token, Position)> {
		if self.peeked_token.is_some() {
			let x = self.peeked_token.clone().unwrap();
			self.peeked_token = None;
			return x;
		}

		match self.running_macro {
		None => {}
		Some((idx, pos)) => {
			if pos >= self.macros[idx].len() {
				self.running_macro = None;
			} else {
				self.running_macro = Some((idx, pos + 1));
				return Some(self.macros[idx][pos].clone());
			}
		}
		}
		
		let mut token_str = String::new();
		let mut initial_pos: Option<Position> = None;
		
		loop {
			let (c, pos) =
				self.next_character(true);
			if let Err(e) = c {
				print_error::<&str>(pos, Some(e.kind()), None);
				if token_str.len() == 0 {
					return None;
				} else {
					if self.launch_macro(&token_str) {
						return self.next();
					} else {
						return Token::from_string(initial_pos.unwrap(), token_str);
					}
				}
			}
			let c = c.unwrap();

			if Token::is_alphabetic(c) || Token::is_numeric(c) || c == '$' {
				if c != '$' {
					token_str.push(c);
				}
				if initial_pos.is_none() {
					initial_pos = Some(pos);
				}
				continue;
			}

			if token_str.len() > 0 {
				self.stash = Some((c, pos));

				if self.launch_macro(&token_str) {
					return self.next();
				} else {
					return Token::from_string(initial_pos.unwrap(), token_str);
				}
			}

			match c {
			'=' => { return Some((Token::Equal, pos)); }
			'.' => { return Some((Token::Dot, pos)); }
			'(' => { return Some((Token::LParan, pos)); }
			')' => { return Some((Token::RParan, pos)); }
			'+' => { return Some((Token::Plus, pos)); }
			'-' => { return Some((Token::Minus, pos)); }
			'*' => { return Some((Token::Star, pos)); }
			',' => { return Some((Token::Comma, pos)); }
			';' => { return Some((Token::SemiColon, pos)); }
			':' => { return Some((Token::Colon, pos)); }
			'/' => {
				let (next_c, next_c_pos) =
					self.next_character(true);
				if let Ok('*') = next_c {
					// We're in a comment
					loop {
						let (c, pos) =
							self.next_character(false);
						match c {
						Ok('*') => {
							let (c, pos) =
								self.next_character(false);
							match c {
							Ok('/') => { break; }
							Ok(_) => {}
							Err(e) => {
								print_error(pos, 
									Some(e.kind()),
									Some("Still in unfinished comment")
								);
								return None;
							}
							}
						}
						Ok(_) => {}
						Err(e) => {
							print_error(pos,
					Some(e.kind()),
					Some("Still in unfinished comment")
							);
							return None;
						}
						}
					}
					return self.next();
				} else if let Ok(next_c) = next_c {
					self.stash = Some((next_c, next_c_pos));
				}
				return Some((Token::Slash, pos));
			}
			'>' => {
				let (next_c, next_c_pos) =
					self.next_character(true);
				if let Ok('=') = next_c {
					return Some((Token::GreaterEqual, pos));
				} else if let Ok(next_c) = next_c {
					self.stash = Some((next_c, next_c_pos));
				}
				return Some((Token::Greater, pos));
			}
			'<' => {
				let (next_c, next_c_pos) =
					self.next_character(true);
				if let Ok('=') = next_c {
					return Some((Token::LessEqual, pos));
				} else if let Ok('>') = next_c {
					return Some((Token::NotEqual, pos));
				} else if let Ok(next_c) = next_c {
					self.stash = Some((next_c, next_c_pos));
				}
				return Some((Token::Less, pos));
			}
			'\'' => {
				loop {
					let (c, pos) =
						self.next_character(false);
					match c {
					Ok('\'') => { break; }
					Ok('\\') => {
						// If it's a sequence
						let (c, pos) = self.next_character(false);
						match c {
						Ok('\\') => { token_str.push('\\'); }
						Ok('\'') => { token_str.push('\''); }
						Ok('t') | Ok('T') => { token_str.push('\t'); }
						Ok('r') | Ok('R') => { token_str.push('\r'); }
						Ok('n') | Ok('N') => { token_str.push('\n'); }
						Ok(c) => {
							token_str.push('\\');
							token_str.push(c);
						}
						Err(e) => {
							print_error(pos,
								Some(e.kind()),
								Some("Still in unfinished string")
							);
							return None;
						}
						}
					}
					Ok(c) => { token_str.push(c); }
					Err(e) => {
						print_error(pos,
							Some(e.kind()),
							Some("Still in unfinished string")
						);
						return None;
					}
					}
				}
				return Some((Token::String(token_str), pos));
			}

			_ => { return self.next(); }
			}
		}
	}
}
