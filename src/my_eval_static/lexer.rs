use crate::my_eval::token::Token;

pub trait Lexer<'a, T> where
    T : Iterator<Item=Token> + 'a {
    fn tokens(&'a self) -> T;
}
