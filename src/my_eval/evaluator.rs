use super::eval_expression::{EvalExpression, EvalBinOpType};

fn eval_bin_op(op: EvalBinOpType, left: i32, right: i32) -> i32 {
    return match op {
        EvalBinOpType::Plus => left + right,
        EvalBinOpType::Minus => left - right,
        EvalBinOpType::Times => left * right,
        EvalBinOpType::Div => left / right,
    };
}

pub fn evaluate(expr: &EvalExpression) -> i32 {
    return match expr {
        EvalExpression::BinOp(e) => {
            let left = evaluate(e.get_left());
            let right = evaluate(e.get_right());
            eval_bin_op(e.get_op(), left, right)
        },
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
        assert_eq!(5, evaluate(&EvalExpression::new_binop(
            EvalBinOpType::Plus,
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1))));
    }

    #[test]
    fn evaluate_bin_op_minus() {
        assert_eq!(3, evaluate(&EvalExpression::new_binop(
            EvalBinOpType::Minus,
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1))));
    }

    #[test]
    fn evaluate_bin_op_times() {
        assert_eq!(8, evaluate(&EvalExpression::new_binop(
            EvalBinOpType::Times,
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(2))));
    }

    #[test]
    fn evaluate_bin_op_div() {
        assert_eq!(3, evaluate(&EvalExpression::new_binop(
            EvalBinOpType::Div,
            EvalExpression::new_value_box(6),
            EvalExpression::new_value_box(2))));
    }
}
