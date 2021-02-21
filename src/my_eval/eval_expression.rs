use std::fmt;
use std::fmt::Debug;
use fmt::Formatter;

#[derive(Eq, PartialEq, Debug)]
pub enum EvalExpression {
    Plus(EvalBinaryExpression),
    Minus(EvalBinaryExpression),
    Times(EvalBinaryExpression),
    Div(EvalBinaryExpression),
    Value(i32)
}

impl EvalExpression {
    pub fn new_value_box(value: i32) -> Box<EvalExpression> {
        return Box::new(EvalExpression::Value(value))
    }

    pub fn new_plus(left: Box<EvalExpression>,
                    right: Box<EvalExpression>) -> EvalExpression {
        return EvalExpression::Plus(EvalBinaryExpression { left, right })
    }

    pub fn new_minus(left: Box<EvalExpression>,
                     right: Box<EvalExpression>) -> EvalExpression {
        return EvalExpression::Minus(EvalBinaryExpression { left, right })
    }

    pub fn new_times(left: Box<EvalExpression>,
                     right: Box<EvalExpression>) -> EvalExpression {
        return EvalExpression::Times(EvalBinaryExpression { left, right })
    }

    pub fn new_div(left: Box<EvalExpression>,
                   right: Box<EvalExpression>) -> EvalExpression {
        return EvalExpression::Div(EvalBinaryExpression { left, right })
    }

    pub fn new_plus_box(left: Box<EvalExpression>,
                        right: Box<EvalExpression>) -> Box<EvalExpression> {
        return Box::new(EvalExpression::Plus(EvalBinaryExpression { left, right }))
    }

    pub fn new_minus_box(left: Box<EvalExpression>,
                        right: Box<EvalExpression>) -> Box<EvalExpression> {
        return Box::new(EvalExpression::Minus(EvalBinaryExpression { left, right }))
    }

    pub fn new_times_box(left: Box<EvalExpression>,
                        right: Box<EvalExpression>) -> Box<EvalExpression> {
        return Box::new(EvalExpression::Times(EvalBinaryExpression { left, right }))
    }

    pub fn new_div_box(left: Box<EvalExpression>,
                        right: Box<EvalExpression>) -> Box<EvalExpression> {
        return Box::new(EvalExpression::Div(EvalBinaryExpression { left, right }))
    }
}

pub struct EvalBinaryExpression {
    left: Box<EvalExpression>,
    right: Box<EvalExpression>
}

impl EvalBinaryExpression {
    pub fn new(left: Box<EvalExpression>,
               right: Box<EvalExpression>) -> EvalBinaryExpression {
        return EvalBinaryExpression {left, right};
    }

    pub fn get_left(&self) -> &EvalExpression {
        return self.left.as_ref();
    }

    pub fn get_right(&self) -> &EvalExpression {
        return self.right.as_ref();
    }
}

impl PartialEq for EvalBinaryExpression {
    fn eq(&self, other: &EvalBinaryExpression) -> bool {
        return *self.get_left() == *other.get_left()
            && *self.get_right() == *other.get_right();
    }
}

impl Eq for EvalBinaryExpression {}

impl Debug for EvalBinaryExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Binary")
            .field("left", self.get_left())
            .field("right", self.get_right())
            .finish()
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
        let a1: Box<EvalExpression> = EvalExpression::new_value_box(4);
        let a2: Box<EvalExpression> = EvalExpression::new_value_box(4);
        let a3: Box<EvalExpression> = EvalExpression::new_value_box(8);
        assert_eq!(*a1, *a1);
        assert_eq!(*a1, *a2);
        assert_ne!(*a1, *a3);
    }

    #[test]
    fn eval_binary_expression() {
        let root = EvalBinaryExpression::new(
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1));
        assert_eq!(EvalExpression::Value(4), *root.get_left());
        assert_eq!(EvalExpression::Value(1), *root.get_right());
    }

    #[test]
    fn eval_plus_expression_debug_string() {
        assert_eq!("Plus(Binary { left: Value(4), right: Value(1) })", format!("{:?}", EvalExpression::new_plus(
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1))));
    }

    #[test]
    fn eval_minus_expression_debug_string() {
        assert_eq!("Minus(Binary { left: Value(4), right: Value(1) })", format!("{:?}", EvalExpression::new_minus(
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1))));
    }

    #[test]
    fn eval_times_expression_debug_string() {
        assert_eq!("Times(Binary { left: Value(4), right: Value(1) })", format!("{:?}", EvalExpression::new_times(
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1))));
    }

    #[test]
    fn eval_div_expression_debug_string() {
        assert_eq!("Div(Binary { left: Value(4), right: Value(1) })", format!("{:?}", EvalExpression::new_div(
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1))));
    }

    #[test]
    fn eval_binary_expression_eq() {
        let a1: Box<EvalExpression> = EvalExpression::new_plus_box(
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1));
        let a2: Box<EvalExpression> = EvalExpression::new_plus_box(
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1));
        let a3: Box<EvalExpression> = EvalExpression::new_minus_box(
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1));
        assert_eq!(*a1, *a1);
        assert_eq!(*a1, *a2);
        assert_ne!(*a1, *a3);
    }

    #[test]
    fn eval_expression_eq() {
        let a1: Box<EvalExpression> = EvalExpression::new_plus_box(
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1));
        let a2: Box<EvalExpression> = EvalExpression::new_value_box(4);
        assert_ne!(*a1, *a2);
        assert_ne!(*a2, *a1);
    }
}