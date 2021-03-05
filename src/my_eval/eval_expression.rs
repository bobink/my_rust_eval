use std::fmt::Debug;

#[derive(Eq, PartialEq, Debug)]
pub enum EvalExpression {
    Plus(Box<EvalExpression>, Box<EvalExpression>),
    Minus(Box<EvalExpression>, Box<EvalExpression>),
    Times(Box<EvalExpression>, Box<EvalExpression>),
    Div(Box<EvalExpression>, Box<EvalExpression>),
    Value(i32)
}

impl EvalExpression {
    pub fn value_box(value: i32) -> Box<EvalExpression> {
        return Box::new(EvalExpression::Value(value))
    }

    pub fn plus_box(left: Box<EvalExpression>,
                    right: Box<EvalExpression>) -> Box<EvalExpression> {
        return Box::new(EvalExpression::Plus(left, right))
    }

    pub fn minus_box(left: Box<EvalExpression>,
                     right: Box<EvalExpression>) -> Box<EvalExpression> {
        return Box::new(EvalExpression::Minus(left, right))
    }

    pub fn times_box(left: Box<EvalExpression>,
                     right: Box<EvalExpression>) -> Box<EvalExpression> {
        return Box::new(EvalExpression::Times(left, right))
    }

    pub fn div_box(left: Box<EvalExpression>,
                   right: Box<EvalExpression>) -> Box<EvalExpression> {
        return Box::new(EvalExpression::Div(left, right))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_value_debug_string() {
        assert_eq!("Value(4)", format!("{:?}", EvalExpression::Value(4)));
    }

    #[test]
    fn eval_value_eq() {
        let a1: Box<EvalExpression> = EvalExpression::value_box(4);
        let a2: Box<EvalExpression> = EvalExpression::value_box(4);
        let a3: Box<EvalExpression> = EvalExpression::value_box(8);
        assert_eq!(*a1, *a1);
        assert_eq!(*a1, *a2);
        assert_ne!(*a1, *a3);
    }

    #[test]
    fn eval_plus_expression_debug_string() {
        assert_eq!("Plus(Value(4), Value(1))", format!("{:?}", EvalExpression::Plus(
            EvalExpression::value_box(4),
            EvalExpression::value_box(1))));
    }

    #[test]
    fn eval_minus_expression_debug_string() {
        assert_eq!("Minus(Value(4), Value(1))", format!("{:?}", EvalExpression::Minus(
            EvalExpression::value_box(4),
            EvalExpression::value_box(1))));
    }

    #[test]
    fn eval_times_expression_debug_string() {
        assert_eq!("Times(Value(4), Value(1))", format!("{:?}", EvalExpression::Times(
            EvalExpression::value_box(4),
            EvalExpression::value_box(1))));
    }

    #[test]
    fn eval_div_expression_debug_string() {
        assert_eq!("Div(Value(4), Value(1))", format!("{:?}", EvalExpression::Div(
            EvalExpression::value_box(4),
            EvalExpression::value_box(1))));
    }

    #[test]
    fn eval_binary_expression_eq() {
        let a1: Box<EvalExpression> = EvalExpression::plus_box(
            EvalExpression::value_box(4),
            EvalExpression::value_box(1));
        let a2: Box<EvalExpression> = EvalExpression::plus_box(
            EvalExpression::value_box(4),
            EvalExpression::value_box(1));
        let a3: Box<EvalExpression> = EvalExpression::minus_box(
            EvalExpression::value_box(4),
            EvalExpression::value_box(1));
        assert_eq!(*a1, *a1);
        assert_eq!(*a1, *a2);
        assert_ne!(*a1, *a3);
    }

    #[test]
    fn eval_expression_eq() {
        let a1: Box<EvalExpression> = EvalExpression::plus_box(
            EvalExpression::value_box(4),
            EvalExpression::value_box(1));
        let a2: Box<EvalExpression> = EvalExpression::value_box(4);
        assert_ne!(*a1, *a2);
        assert_ne!(*a2, *a1);
    }
}