pub mod lexer;
pub mod lexer_impl;
pub mod parser;
pub mod reader;
pub mod reader_impl;

use parser::parse;
use lexer_impl::lexer_impl;
use reader_impl::string_reader;
use crate::my_eval::evaluator::evaluate;

pub fn my_eval(s: &str) -> i32 {
    let lexer = lexer_impl(string_reader(s));
    let p = parse(&lexer);
    return evaluate(p.as_ref());
}

#[cfg(test)]
mod tests {
    use super::my_eval;

    #[test]
    fn test_eleven_plus_three() {
        assert_eq!(14, my_eval("11 + 3"));
    }

    #[test]
    fn test_three_times_six() {
        assert_eq!(18, my_eval("3 * 6"));
    }

    #[test]
    #[should_panic]
    fn test_none() {
        my_eval("");
    }

    #[test]
    #[should_panic]
    fn test_two_three() {
        my_eval("2 3");
    }

    #[test]
    #[should_panic]
    fn test_plus() {
        my_eval("+");
    }

    #[test]
    fn test_forty_two() {
        assert_eq!(42, my_eval("42"));
    }

    #[test]
    fn test_two_plus_tree_times_four() {
        assert_eq!(14, my_eval("2 + 3 * 4"));
    }

    #[test]
    fn test_two_times_tree_plus_four() {
        assert_eq!(10, my_eval("2 * 3 + 4"));
    }

    #[test]
    fn test_p_two_plus_tree_p_times_four() {
        assert_eq!(20, my_eval("(2 + 3) * 4"));
    }

    #[test]
    fn test_three_minus_five() {
        assert_eq!(-2, my_eval("3 - 5"));
    }

    #[test]
    fn test_big_expression() {
        let str = "5 + 9 * 89 - 23 + 65 * 4 + 42 - 23 * 2 * 3";
        assert_eq!(947, my_eval(str));
    }
}
