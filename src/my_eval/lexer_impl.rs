use super::lexer::{Lexer, LexerIterator};
use super::token::{Token, TokenType};
use super::reader::{Reader, ReaderIterator};

struct LexerImpl {
    reader: Box<dyn Reader>
}

struct LexerIteratorImpl<'a> {
    last: char,
    it: Box<dyn ReaderIterator + 'a>
}

impl LexerImpl {
    fn new(reader: Box<dyn Reader>) -> LexerImpl {
        LexerImpl {
            reader
        }
    }
}

impl Lexer for LexerImpl {
    fn tokens<'a>(&'a self) -> Box<dyn LexerIterator + 'a> {
        return Box::new(LexerIteratorImpl::new(self.reader.chars()));
    }
}

impl<'a> LexerIteratorImpl<'a> {
    fn new(it: Box<dyn ReaderIterator + 'a>) -> LexerIteratorImpl<'a> {
        LexerIteratorImpl {
            last: ' ',
            it
        }
    }
}

fn token(ttype : TokenType) -> Option<Token>{
    return Some(Token::new_token(ttype));
}

fn to_10_digit(c: char) -> u32 {
    c.to_digit(10).unwrap()
}

impl<'a> LexerIteratorImpl<'a> {
    fn read_token_value(&mut self, c: char) -> u32 {
        let mut value: u32 = to_10_digit(c);
        loop {
            match self.it.next() {
                Some(tmp) if ('0'..='9').contains(&tmp) => value = value * 10 + to_10_digit(tmp),
                Some(tmp) => {
                    self.last = tmp;
                    break;
                },
                None => break
            }
        }
        return value;
    }
}

impl<'a> LexerIterator for LexerIteratorImpl<'a> {
    fn next(&mut self) -> Option<Token> {
        let mut c = self.last;
        self.last = ' ';

        while c == ' ' {
            match self.it.next() {
                Some(tmp) => c = tmp,
                None => return None
            }
        }
        match c {
            '+' => return token(TokenType::Plus),
            '-' => return token(TokenType::Minus),
            '*' => return token(TokenType::Times),
            '/' => return token(TokenType::Div),
            '(' => return token(TokenType::LeftParenthesis),
            ')' => return token(TokenType::RightParenthesis),
            '0'..='9' => Some(Token::new_value(self.read_token_value(c))),
            _ => panic!(format!("Unsupported character {}", c))
        }
    }
}

pub fn lexer_impl(reader: Box<dyn Reader>) -> Box<dyn Lexer> {
    return Box::new(LexerImpl::new(reader));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_eval::reader_impl::string_reader;
    use std::ops::DerefMut;

    fn string_lexer(s: &str) -> Box<dyn Lexer> {
        return lexer_impl(string_reader(s));
    }

    #[test]
    fn lex_eleven_plus_three() {
        let lexer = string_lexer("11 + 3");
        let mut b = lexer.tokens();
        let tokens = b.deref_mut();
        assert_eq!(Some(Token::new_value(11)), tokens.next());
        assert_eq!(Some(Token::new_token(TokenType::Plus)), tokens.next());
        assert_eq!(Some(Token::new_value(3)), tokens.next());
        assert_eq!(None, tokens.next());
    }

    #[test]
    fn lex_big_whitespace() {
        let lexer = string_lexer("   325      +123*    66");
        let mut b = lexer.tokens();
        let tokens = b.deref_mut();
        assert_eq!(Some(Token::new_value(325)), tokens.next());
        assert_eq!(Some(Token::new_token(TokenType::Plus)), tokens.next());
        assert_eq!(Some(Token::new_value(123)), tokens.next());
        assert_eq!(Some(Token::new_token(TokenType::Times)), tokens.next());
        assert_eq!(Some(Token::new_value(66)), tokens.next());
        assert_eq!(None, tokens.next());
    }

    #[test]
    fn lex_empty() {
        let lexer = string_lexer("   ");
        let mut b = lexer.tokens();
        let tokens = b.deref_mut();
        assert_eq!(None, tokens.next());
    }
}