use super::lexer::Lexer;
use super::reader::Reader;
use crate::my_eval::token::Token;
use std::marker::PhantomData;

pub struct LexerImpl<'a, RIT, R>
    where RIT: Iterator<Item=char> + 'a, R: Reader<'a, RIT> {

    reader: R,
    phantom: PhantomData<&'a RIT>,
}

pub struct LexerIteratorImpl<'a, RIT>
    where RIT: Iterator<Item=char> + 'a {

    last: char,
    it: RIT,
    phantom: PhantomData<&'a RIT>,
}

impl<'a, RIT, R> LexerImpl<'a, RIT, R>
    where RIT: Iterator<Item=char> + 'a, R: Reader<'a, RIT> {

    fn new(reader: R) -> LexerImpl<'a, RIT, R> {
        LexerImpl {
            reader,
            phantom: PhantomData
        }
    }
}

impl<'a, RIT, R> Lexer<'a, LexerIteratorImpl<'a, RIT>> for LexerImpl<'a, RIT, R>
    where RIT: Iterator<Item=char> + 'a, R: Reader<'a, RIT> {

    fn tokens(&'a self) -> LexerIteratorImpl<'a, RIT> {
        return LexerIteratorImpl::new(self.reader.chars());
    }
}

impl<'a, RIT> LexerIteratorImpl<'a, RIT>
    where RIT: Iterator<Item=char> + 'a {

    fn new(it: RIT) -> LexerIteratorImpl<'a, RIT> {
        LexerIteratorImpl {
            last: ' ',
            it,
            phantom: PhantomData
        }
    }
}

fn to_10_digit(c: char) -> i32 {
    c.to_digit(10).unwrap() as i32
}

impl<'a, RIT> LexerIteratorImpl<'a, RIT>
    where RIT: Iterator<Item=char> + 'a {

    fn read_token_value(&mut self, c: char) -> i32 {
        let mut value: i32 = to_10_digit(c);
        loop {
            match self.it.next() {
                Some(tmp) if ('0'..='9').contains(&tmp) => value = value * 10 + to_10_digit(tmp),
                Some(tmp) => {
                    self.last = tmp;
                    break;
                }
                None => break
            }
        }
        return value;
    }
}

impl<'a, RIT> Iterator for LexerIteratorImpl<'a, RIT>
    where RIT: Iterator<Item=char> + 'a {
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
        });
    }
}

pub fn lexer_impl<'a, RIT, R>(reader: R) -> LexerImpl<'a, RIT, R>
    where RIT: Iterator<Item=char> + 'a, R: Reader<'a, RIT> {

    return LexerImpl::new(reader);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::my_eval_static::reader_impl::string_reader;

    fn token_vec_of_string(s: &str) -> Vec<Token> {
        return lexer_impl(string_reader(s)).tokens().collect();
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