use super::token::Token;

pub trait Lexer {
    fn tokens<'a>(&'a self) -> Box<dyn LexerIterator + 'a>;
}

pub trait LexerIterator {
    fn next(&mut self) -> Option<Token>;
}
