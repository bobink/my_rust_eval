use super::lexer::Lexer;
use super::token::Token;
use super::reader::Reader;

struct LexerImpl {
    reader: Box<dyn Reader>
}

struct LexerIteratorImpl<'a> {
    last: char,
    it: Box<dyn Iterator<Item=char> + 'a>
}

impl LexerImpl {
    fn new(reader: Box<dyn Reader>) -> LexerImpl {
        LexerImpl {
            reader
        }
    }
}

impl Lexer for LexerImpl {
    fn tokens(& self) -> Box<dyn Iterator<Item=Token> + '_> {
        return Box::new(LexerIteratorImpl::new(self.reader.chars()));
    }
}

impl<'a> LexerIteratorImpl<'a> {
    fn new(it: Box<dyn Iterator<Item=char> + 'a>) -> LexerIteratorImpl<'a> {
        LexerIteratorImpl {
            last: ' ',
            it
        }
    }
}

fn to_10_digit(c: char) -> i32 {
    c.to_digit(10).unwrap() as i32
}

impl<'a> LexerIteratorImpl<'a> {
    fn read_token_value(&mut self, c: char) -> i32 {
        let mut value: i32 = to_10_digit(c);
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

impl<'a> Iterator for LexerIteratorImpl<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let mut c = self.last;
        self.last = ' ';

        while c == ' ' {
            match self.it.next() {
                Some(tmp) => c = tmp,
                None => return None
            }
        }
        return Some(match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Times,
            '/' => Token::Div,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '0'..='9' => Token::Value(self.read_token_value(c)),
            _ => panic!(format!("Unsupported character {}", c))
        })
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

    fn token_vec_of_string(s: &str) -> Vec<Token> {
        return lexer_impl(string_reader(s)).tokens().deref_mut().collect();
    }

    #[test]
    fn lex_eleven_plus_three() {
        let actual = token_vec_of_string("11 + 3");
        assert_eq!(vec![Token::Value(11), Token::Plus, Token::Value(3)], actual);
    }

    #[test]
    fn lex_big_whitespace() {
        let actual = token_vec_of_string("   325      +123*    66");
        let expected = vec![Token::Value(325), Token::Plus, Token::Value(123),
                            Token::Times, Token::Value(66)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_empty() {
        let actual = token_vec_of_string("   ");
        let expected: Vec<Token> = vec![];
        assert_eq!(expected, actual);
    }
}