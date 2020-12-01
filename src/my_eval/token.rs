#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Plus,
    Minus,
    Times,
    Div,
    LeftParenthesis,
    RightParenthesis,
    Value
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Token {
    ttype: TokenType,
    value: u32
}

impl Token {
    pub fn new_value(value: u32) -> Token {
        Token {
            ttype: TokenType::Value,
            value
        }
    }

    pub fn new_token(ttype: TokenType) -> Token {
        Token {
            ttype,
            value: 0
        }
    }

    pub fn get_type(&self) -> TokenType {
        return self.ttype;
    }

    pub fn get_value(&self) -> u32 {
        return self.value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_value() {
        let t = Token::new_value(42);
        assert_eq!(TokenType::Value, t.get_type());
        assert_eq!(42, t.get_value());
    }

    #[test]
    fn new_token() {
        let t = Token::new_token(TokenType::Plus);
        assert_eq!(TokenType::Plus, t.get_type());
        assert_eq!(0, t.get_value());
    }

}