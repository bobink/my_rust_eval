use super::eval_expression::EvalExpression;

pub fn evaluate(expr: &EvalExpression) -> i32 {
    return match expr {
        EvalExpression::Plus(e) =>
            evaluate(e.get_left()) + evaluate(e.get_right()),
        EvalExpression::Minus(e) =>
            evaluate(e.get_left()) - evaluate(e.get_right()),
        EvalExpression::Times(e) =>
            evaluate(e.get_left()) * evaluate(e.get_right()),
        EvalExpression::Div(e) =>
            evaluate(e.get_left()) / evaluate(e.get_right()),
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
        assert_eq!(5, evaluate(&EvalExpression::new_plus(
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1))));
    }

    #[test]
    fn evaluate_bin_op_minus() {
        assert_eq!(3, evaluate(&EvalExpression::new_minus(
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1))));
    }

    #[test]
    fn evaluate_bin_op_times() {
        assert_eq!(8, evaluate(&EvalExpression::new_times(
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(2))));
    }

    #[test]
    fn evaluate_bin_op_div() {
        assert_eq!(3, evaluate(&EvalExpression::new_div(
            EvalExpression::new_value_box(6),
            EvalExpression::new_value_box(2))));
    }
}
