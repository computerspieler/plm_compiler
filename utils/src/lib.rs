pub mod token;
pub mod lexer;

pub trait KeywordHandler {
    fn is_keyword<'a>(&self, s: &'a str) -> Option<&'static str>;
}

impl<const N: usize> KeywordHandler for [&'static str; N] {
    fn is_keyword<'a>(&self, s: &'a str) -> Option<&'static str>
    {
        let n = self.len();
        for i in 0 .. n {
            if self[i] == s {
                return Some(self[i]);
            }
        }
        None
    }
}

pub trait EOSDetector: Iterator {
	fn reached_eos(&mut self) -> bool;
}
