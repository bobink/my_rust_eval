use super::token::Token;

pub trait Lexer {
    fn tokens(& self) -> Box<dyn Iterator<Item=Token> + '_>;
}

