use super::eval_expression::{EvalExpression, EvalValue, EvalBinOp, EvalBinOpType};

struct EvalExpressionEvaluator {
    result: u32
}

fn eval_bin_op(op: EvalBinOpType, left: u32, right: u32) -> u32 {
    return match op {
        EvalBinOpType::Plus => left + right,
        EvalBinOpType::Minus => left - right,
        EvalBinOpType::Times => left * right,
        EvalBinOpType::Div => left / right,
    };
}

impl super::eval_expression::EvalExpressionVisitor for EvalExpressionEvaluator {
    fn visit_bin_op(&mut self, e: &EvalBinOp) {
        let left = evaluate(e.get_left());
        let right = evaluate(e.get_right());
        self.result = eval_bin_op(e.get_op(), left, right);
    }

    fn visit_value(&mut self, e: &EvalValue) {
        self.result = e.get_value();
    }
}

pub fn evaluate(expr: &dyn EvalExpression) -> u32 {
    let mut visitor = EvalExpressionEvaluator {result: 0};
    expr.accept(&mut visitor);
    return visitor.result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value() {
        assert_eq!(4, evaluate(&EvalValue::new(4)));
    }

    #[test]
    fn evaluate_bin_op_plus() {
        assert_eq!(5, evaluate(&EvalBinOp::new(
            EvalBinOpType::Plus,
            EvalValue::new_box(4),
            EvalValue::new_box(1))));
    }

    #[test]
    fn evaluate_bin_op_minus() {
        assert_eq!(3, evaluate(&EvalBinOp::new(
            EvalBinOpType::Minus,
            EvalValue::new_box(4),
            EvalValue::new_box(1))));
    }

    #[test]
    fn evaluate_bin_op_times() {
        assert_eq!(8, evaluate(&EvalBinOp::new(
            EvalBinOpType::Times,
            EvalValue::new_box(4),
            EvalValue::new_box(2))));
    }

    #[test]
    fn evaluate_bin_op_div() {
        assert_eq!(3, evaluate(&EvalBinOp::new(
            EvalBinOpType::Div,
            EvalValue::new_box(6),
            EvalValue::new_box(2))));
    }
}
