
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Token {
    Plus,
    Minus,
    Times,
    Div,
    LeftParenthesis,
    RightParenthesis,
    Value(i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_plus_debug_string() {
        assert_eq!("Plus", format!("{:?}", Token::Plus));
    }

    #[test]
    fn new_token() {
        assert_eq!("Value(42)", format!("{:?}", Token::Value(42)));
    }
}