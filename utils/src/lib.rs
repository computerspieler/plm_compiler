pub mod token;

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