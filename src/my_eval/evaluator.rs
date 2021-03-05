use super::eval_expression::EvalExpression;
use std::ops::Deref;

pub fn evaluate(expr: &EvalExpression) -> i32 {
    return match expr {
        EvalExpression::Plus(left, right) =>
            evaluate(left.deref()) + evaluate(right.deref()),
        EvalExpression::Minus(left, right) =>
            evaluate(left.deref()) - evaluate(right.deref()),
        EvalExpression::Times(left, right) =>
            evaluate(left.deref()) * evaluate(right.deref()),
        EvalExpression::Div(left, right) =>
            evaluate(left.deref()) / evaluate(right.deref()),
        EvalExpression::Value(v) => *v
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(4, evaluate(&EvalExpression::Value(4)));
    }

    #[test]
    fn evaluate_bin_op_plus() {
        assert_eq!(5, evaluate(&EvalExpression::Plus(
            EvalExpression::value_box(4),
            EvalExpression::value_box(1))));
    }

    #[test]
    fn evaluate_bin_op_minus() {
        assert_eq!(3, evaluate(&EvalExpression::Minus(
            EvalExpression::value_box(4),
            EvalExpression::value_box(1))));
    }

    #[test]
    fn evaluate_bin_op_times() {
        assert_eq!(8, evaluate(&EvalExpression::Times(
            EvalExpression::value_box(4),
            EvalExpression::value_box(2))));
    }

    #[test]
    fn evaluate_bin_op_div() {
        assert_eq!(3, evaluate(&EvalExpression::Div(
            EvalExpression::value_box(6),
            EvalExpression::value_box(2))));
    }
}
