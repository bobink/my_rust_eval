use super::eval_expression::{EvalExpression, EvalBinOp, EvalBinOpType, EvalValue};
use super::lexer::{Lexer, LexerIterator};
use super::token::{Token, TokenType};

struct ReduceResult {
    op: EvalBinOpType,
    term: Box<dyn EvalExpression>,
    right: Option<Box<ReduceResult>>
}

impl ReduceResult {
    fn new_box(op: EvalBinOpType, term: Box<dyn EvalExpression>, right: Option<Box<ReduceResult>>) -> Box<ReduceResult> {
        return Box::new(ReduceResult {op, term, right});
    }
}

struct TokenStack<'a> {
    tokens: Box<dyn LexerIterator + 'a>,
    head: Option<Token>
}

impl<'b> TokenStack<'b> {
    fn new<'a>(tokens: Box<dyn LexerIterator + 'a>) -> TokenStack<'a> {
        return TokenStack {tokens, head: None}
    }

    fn head(&mut self) -> Option<Token> {
        if self.head == None {
            self.head = self.tokens.next();
        }
        return self.head;
    }

    fn pop(&mut self) -> Option<Token> {
        let result = self.head();
        self.head = None;
        return result;
    }
}

/*
Grammar:
add = mul add'
add' = ε
add' = "+" mul add'
add' = "-" mul add'

mul = term mul'
mul' = ε
mul' = "*" term mul'
mul' = "/" term mul'

term = <num>
term = "(" add ")"
*/

fn parse_add(stack: &mut TokenStack) -> Box<dyn EvalExpression> {
    let mut left = parse_mul(stack);
    let mut right = parse_add_p(stack);
    loop {
        match right {
            Some(r) => {
                left = EvalBinOp::new_box(r.op, left, r.term);
                right = r.right;
            }
            None => return left
        }
    }
}

fn parse_add_p(stack: &mut TokenStack) -> Option<Box<ReduceResult>> {
    match stack.head() {
        None => return None,
        Some(t) => {
            match t.get_type() {
                TokenType::Plus | TokenType::Minus => {}
                _ => return None
            }
        }
    }
    let t = stack.pop().unwrap().get_type();
    let bin_op_t = parse_add_op(t);
    let term = parse_mul(stack);
    let expr = parse_add_p(stack);
    return Some(ReduceResult::new_box(bin_op_t, term, expr));
}

fn unexpected_token(expected: TokenType, actual: TokenType) -> String {
    return format!("Unexpected token. Expected: {:?}. Got: {:?}", expected, actual);
}

fn parse_add_op(t: TokenType) -> EvalBinOpType {
    return match t {
        TokenType::Plus => EvalBinOpType::Plus,
        TokenType::Minus => EvalBinOpType::Minus,
        _ => panic!(unexpected_token(TokenType::Plus, t))
    }
}

fn parse_mul(stack: &mut TokenStack) -> Box<dyn EvalExpression> {
    let mut left = parse_term(stack);
    let mut right = parse_mul_p(stack);
    loop {
        match right {
            Some(r) => {
                left = EvalBinOp::new_box(r.op, left, r.term);
                right = r.right;
            }
            None => return left
        }
    }
}

fn parse_mul_p(stack: &mut TokenStack) -> Option<Box<ReduceResult>> {
    match stack.head() {
        None => return None,
        Some(t) => match t.get_type() {
            TokenType::Times | TokenType::Div => {},
            _ => return None
        }
    }
    let t = stack.pop().unwrap().get_type();
    let bin_op_t = parse_mul_op(t);
    let term = parse_term(stack);
    let expr = parse_mul_p(stack);
    return Some(ReduceResult::new_box(bin_op_t, term, expr));
}

fn parse_mul_op(t: TokenType) -> EvalBinOpType {
    return match t {
        TokenType::Times => EvalBinOpType::Times,
        TokenType::Div => EvalBinOpType::Div,
        _ => panic!(unexpected_token(TokenType::Times, t))
    }
}

fn parse_term(stack: &mut TokenStack) -> Box<dyn EvalExpression> {
    let n = next_token(stack);
    return match n.get_type() {
        TokenType::Value => EvalValue::new_box(n.get_value()),
        TokenType::LeftParenthesis => {
            let expr = parse_add(stack);
            let n2 = next_token(stack);
            match n2.get_type() {
                TokenType::RightParenthesis => expr,
                t => panic!(unexpected_token(TokenType::RightParenthesis, t))
            }
        },
        t => panic!(unexpected_token(TokenType::Value, t))
    }
}

fn next_token(stack: &mut TokenStack) -> Token {
    match stack.pop() {
        Some(t) => return t,
        None => panic!("Unexpected end of token")
    }
}

pub fn parse(lexer: &dyn Lexer) -> Box<dyn EvalExpression> {
    let mut stack = TokenStack::new(lexer.tokens());
    let result = parse_add(&mut stack);
    match stack.head() {
        Some(t) => panic!(format!("Unexpected token: {:?}", t)),
        None => return result
    }
}