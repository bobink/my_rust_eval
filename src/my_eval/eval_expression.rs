use std::fmt;
use std::fmt::Debug;
use fmt::Formatter;

#[derive(Eq, PartialEq, Debug)]
pub enum EvalExpression {
    BinOp(EvalBinOp),
    Value(i32)
}

impl EvalExpression {
    pub fn new_value_box(value: i32) -> Box<EvalExpression> {
        return Box::new(EvalExpression::Value(value))
    }

    pub fn new_binop(op: EvalBinOpType,
                     left: Box<EvalExpression>,
                     right: Box<EvalExpression>) -> EvalExpression {
        return EvalExpression::BinOp(EvalBinOp { op, left, right })
    }

    pub fn new_binop_box(op: EvalBinOpType,
                   left: Box<EvalExpression>,
                   right: Box<EvalExpression>) -> Box<EvalExpression> {
        return Box::new(EvalExpression::BinOp(EvalBinOp { op, left, right }))
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum EvalBinOpType {
    Plus,
    Minus,
    Times,
    Div
}

pub struct EvalBinOp {
    op: EvalBinOpType,
    left: Box<EvalExpression>,
    right: Box<EvalExpression>
}

impl EvalBinOp {
    pub fn new(op: EvalBinOpType,
               left: Box<EvalExpression>,
               right: Box<EvalExpression>) -> EvalBinOp {
        return EvalBinOp{op, left, right};
    }

    pub fn get_op(&self) -> EvalBinOpType {
        return self.op;
    }

    pub fn get_left(&self) -> &EvalExpression {
        return self.left.as_ref();
    }

    pub fn get_right(&self) -> &EvalExpression {
        return self.right.as_ref();
    }
}

impl PartialEq for EvalBinOp {
    fn eq(&self, other: &EvalBinOp) -> bool {
        return self.get_op() == other.get_op()
            && *self.get_left() == *other.get_left()
            && *self.get_right() == *other.get_right();
    }
}

impl Eq for EvalBinOp {}

impl Debug for EvalBinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("EvalBinOp")
            .field("op", &self.get_op())
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
    fn eval_bin_op() {
        let root = EvalBinOp::new(EvalBinOpType::Plus,
                                  EvalExpression::new_value_box(4),
                                  EvalExpression::new_value_box(1));
        assert_eq!(EvalBinOpType::Plus, root.get_op());
        assert_eq!(EvalExpression::Value(4), *root.get_left());
        assert_eq!(EvalExpression::Value(1), *root.get_right());
    }

    #[test]
    fn eval_bin_op_debug_string() {
        assert_eq!("EvalBinOp { op: Plus, left: Value(4), right: Value(1) }", format!("{:?}", EvalBinOp::new(
            EvalBinOpType::Plus,
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1))));
    }

    #[test]
    fn eval_bin_op_eq() {
        let a1: Box<EvalExpression> = EvalExpression::new_binop_box(
            EvalBinOpType::Plus,
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1));
        let a2: Box<EvalExpression> = EvalExpression::new_binop_box(
            EvalBinOpType::Plus,
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1));
        let a3: Box<EvalExpression> = EvalExpression::new_binop_box(
            EvalBinOpType::Minus,
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1));
        assert_eq!(*a1, *a1);
        assert_eq!(*a1, *a2);
        assert_ne!(*a1, *a3);
    }

    #[test]
    fn eval_expression_eq() {
        let a1: Box<EvalExpression> = EvalExpression::new_binop_box(
            EvalBinOpType::Plus,
            EvalExpression::new_value_box(4),
            EvalExpression::new_value_box(1));
        let a2: Box<EvalExpression> = EvalExpression::new_value_box(4);
        assert_ne!(*a1, *a2);
        assert_ne!(*a2, *a1);
    }
}