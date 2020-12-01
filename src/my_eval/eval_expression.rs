use std::fmt;
use std::fmt::Debug;
use fmt::Formatter;

pub trait EvalExpressionVisitor {
    fn visit_bin_op(&mut self, eval_bin_op: &EvalBinOp);
    fn visit_value(&mut self, eval_value: &EvalValue);
}

pub trait EvalExpression : Debug {
    fn accept(&self, visitor: &mut dyn EvalExpressionVisitor);

    fn as_debug(&self) -> &dyn Debug;
}

struct EvalExpressionEq<'a> {
    left: &'a(dyn EvalExpression + 'a),
    result: bool
}

impl<'a> EvalExpressionVisitor for EvalExpressionEq<'a> {
    fn visit_bin_op(&mut self, right: &EvalBinOp) {
        self.result = *right == *self.left;
    }

    fn visit_value(&mut self, right: &EvalValue) {
        self.result = *right == *self.left;
    }
}

impl<'a, 'b> PartialEq<dyn EvalExpression + 'a> for dyn EvalExpression + 'b {
    fn eq(&self, right: &(dyn EvalExpression + 'a)) -> bool {
        let mut visitor: EvalExpressionEq = EvalExpressionEq{ left: self, result: false };
        right.accept(&mut visitor);
        return visitor.result;
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
    left: Box<dyn EvalExpression>,
    right: Box<dyn EvalExpression>
}

impl EvalBinOp {
    pub fn new(op: EvalBinOpType,
               left: Box<dyn EvalExpression>,
               right: Box<dyn EvalExpression>) -> EvalBinOp {
        return EvalBinOp { op, left, right }
    }

    pub fn new_box(op: EvalBinOpType,
                   left: Box<dyn EvalExpression>,
                   right: Box<dyn EvalExpression>) -> Box<EvalBinOp> {
        return Box::new(EvalBinOp { op, left, right })
    }

    pub fn get_op(&self) -> EvalBinOpType {
        return self.op;
    }

    pub fn get_left<'a>(&'a self) -> &'a dyn EvalExpression {
        return self.left.as_ref();
    }

    pub fn get_right<'a>(&'a self) -> &'a dyn EvalExpression{
        return self.right.as_ref();
    }
}

struct EvalBinOpEq<'a> {
    left: &'a EvalBinOp,
    result: bool
}

impl<'a> EvalExpressionVisitor for EvalBinOpEq<'a> {
    fn visit_bin_op(&mut self, right: &EvalBinOp) {
        self.result = self.left == right;
    }

    fn visit_value(&mut self, _eval_value: &EvalValue) {
        self.result = false;
    }
}

impl EvalExpression for EvalBinOp {
    fn accept(&self, visitor: &mut dyn EvalExpressionVisitor) {
        visitor.visit_bin_op(self);
    }

    // Didn't find how to directly upcast in Debug. It seems that it's not currently supported
    // by the rust language right now
    fn as_debug(&self) -> &dyn Debug {
        return self;
    }
}

impl PartialEq for EvalBinOp {
    fn eq(&self, other: &EvalBinOp) -> bool {
        return self.get_op() == other.get_op()
            && *self.get_left() == *other.get_left()
            && *self.get_right() == *other.get_right();
    }
}

impl<'a> PartialEq<dyn EvalExpression + 'a> for EvalBinOp {
    fn eq(&self, right: &(dyn EvalExpression + 'a)) -> bool {
        let mut visitor: EvalBinOpEq = EvalBinOpEq{ left: self, result: false };
        right.accept(&mut visitor);
        return visitor.result;
    }
}

impl PartialEq<EvalBinOp> for dyn EvalExpression {
    fn eq(&self, right: &EvalBinOp) -> bool {
        return *right == *self;
    }
}

impl Debug for EvalBinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("EvalBinOp")
            .field("op", &self.get_op())
            .field("left", self.get_left().as_debug())
            .field("right", self.get_right().as_debug())
            .finish()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct EvalValue {
    value: u32
}

impl EvalValue {
    pub fn new(value: u32) -> EvalValue {
        return EvalValue { value }
    }

    pub fn new_box(value: u32) -> Box<EvalValue> {
        return Box::new(EvalValue { value })
    }

    pub fn get_value(&self) -> u32 {
        return self.value;
    }
}

struct EvalValueEq<'a> {
    left: &'a EvalValue,
    result: bool
}

impl<'a> EvalExpressionVisitor for EvalValueEq<'a> {
    fn visit_bin_op(&mut self, _right: &EvalBinOp) {
        self.result = false;
    }

    fn visit_value(&mut self, right: &EvalValue) {
        self.result = self.left == right;
    }
}

impl<'a> PartialEq<dyn EvalExpression + 'a> for EvalValue {
    fn eq(&self, right: &(dyn EvalExpression + 'a)) -> bool {
        let mut visitor: EvalValueEq = EvalValueEq{ left: self, result: false };
        right.accept(&mut visitor);
        return visitor.result;
    }
}

impl PartialEq<EvalValue> for dyn EvalExpression {
    fn eq(&self, right: &EvalValue) -> bool {
        return *right == *self;
    }
}

impl EvalExpression for EvalValue {
    fn accept(&self, visitor: &mut dyn EvalExpressionVisitor) {
        visitor.visit_value(self);
    }

    fn as_debug(&self) -> &dyn Debug {
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_value() {
        assert_eq!(4, EvalValue::new(4).get_value());
        assert_eq!(EvalValue::new(4), EvalValue::new(4));
        assert_ne!(EvalValue::new(4), EvalValue::new(8));
    }

    #[test]
    fn eval_value_debug_string() {
        assert_eq!("EvalValue { value: 4 }", format!("{:?}", EvalValue::new(4)));
    }

    #[test]
    fn eval_value_eq() {
        let a1: Box<dyn EvalExpression> = EvalValue::new_box(4);
        let a2: Box<dyn EvalExpression> = EvalValue::new_box(4);
        let a3: Box<dyn EvalExpression> = EvalValue::new_box(8);
        assert_eq!(*a1, *a1);
        assert_eq!(*a1, *a2);
        assert_ne!(*a1, *a3);
    }

    #[test]
    fn eval_bin_op() {
        let root = EvalBinOp::new(EvalBinOpType::Plus,
                                  EvalValue::new_box(4),
                                  EvalValue::new_box(1));
        assert_eq!(EvalBinOpType::Plus, root.get_op());
        assert_eq!(EvalValue::new(4), *root.get_left());
        assert_eq!(EvalValue::new(1), *root.get_right());
    }

    #[test]
    fn eval_bin_op_debug_string() {
        assert_eq!("EvalBinOp { op: Plus, left: EvalValue { value: 4 }, right: EvalValue { value: 1 } }", format!("{:?}", EvalBinOp::new(
            EvalBinOpType::Plus,
            EvalValue::new_box(4),
            EvalValue::new_box(1))));
    }

    #[test]
    fn eval_bin_op_eq() {
        let a1: Box<dyn EvalExpression> = EvalBinOp::new_box(
            EvalBinOpType::Plus,
            EvalValue::new_box(4),
            EvalValue::new_box(1));
        let a2: Box<dyn EvalExpression> = EvalBinOp::new_box(
            EvalBinOpType::Plus,
            EvalValue::new_box(4),
            EvalValue::new_box(1));
        let a3: Box<dyn EvalExpression> = EvalBinOp::new_box(
            EvalBinOpType::Minus,
            EvalValue::new_box(4),
            EvalValue::new_box(1));
        assert_eq!(*a1, *a1);
        assert_eq!(*a1, *a2);
        assert_ne!(*a1, *a3);
    }

    #[test]
    fn eval_expression_eq() {
        let a1: Box<dyn EvalExpression> = EvalBinOp::new_box(
            EvalBinOpType::Plus,
            EvalValue::new_box(4),
            EvalValue::new_box(1));
        let a2: Box<dyn EvalExpression> = EvalValue::new_box(4);
        assert_ne!(*a1, *a2);
        assert_ne!(*a2, *a1);
    }
}