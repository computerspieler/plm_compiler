pub mod ast;
pub mod il_builder;
pub mod keywords;
pub mod parser;
pub mod parser_macros;
pub mod preprocessor_parser;
pub mod lexer;
pub mod token;

pub trait EOSDetector: Iterator {
	fn reached_eos(&mut self) -> bool;
}
