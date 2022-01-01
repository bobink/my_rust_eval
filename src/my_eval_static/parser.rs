use crate::my_eval::eval_expression::EvalExpression;
use crate::my_eval::token::Token;
use crate::my_eval_static::lexer::Lexer;
use std::marker::PhantomData;

enum ReduceResultType {
    Plus,
    Minus,
    Times,
    Div,
}

struct ReduceResult {
    op: ReduceResultType,
    term: Box<EvalExpression>,
    right: Option<Box<ReduceResult>>,
}

impl ReduceResult {
    fn new_box(
        op: ReduceResultType,
        term: Box<EvalExpression>,
        right: Option<Box<ReduceResult>>,
    ) -> Box<ReduceResult> {
        return Box::new(ReduceResult { op, term, right });
    }
}

struct TokenStack<'a, T>
    where T: Iterator<Item=Token> + 'a {
    tokens: T,
    head: Option<Token>,
    phantom: PhantomData<&'a T>,
}

impl<'b, T> TokenStack<'b, T>
    where T: Iterator<Item=Token> + 'b {
    fn new<'a>(tokens: T) -> TokenStack<'a, T> {
        return TokenStack { tokens, head: None, phantom: PhantomData };
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

fn reduce_to_eval_expression(
    op: ReduceResultType,
    left: Box<EvalExpression>,
    right: Box<EvalExpression>,
) -> Box<EvalExpression> {
    return match op {
        ReduceResultType::Plus => EvalExpression::plus_box(left, right),
        ReduceResultType::Minus => EvalExpression::minus_box(left, right),
        ReduceResultType::Times => EvalExpression::times_box(left, right),
        ReduceResultType::Div => EvalExpression::div_box(left, right),
    };
}

fn parse_add<'a, T>(stack: &mut TokenStack<'a, T>) -> Box<EvalExpression>
    where T: Iterator<Item=Token> + 'a {
    let mut left = parse_mul(stack);
    let mut right = parse_add_p(stack);
    loop {
        match right {
            Some(r) => {
                left = reduce_to_eval_expression(r.op, left, r.term);
                right = r.right;
            }
            None => return left,
        }
    }
}

fn parse_add_p<'a, T>(stack: &mut TokenStack<'a, T>) -> Option<Box<ReduceResult>>
    where T: Iterator<Item=Token> + 'a {
    match stack.head() {
        Some(Token::Plus) | Some(Token::Minus) => {}
        _ => return None,
    }
    let t = stack.pop().unwrap();
    let bin_op_t = parse_add_op(t);
    let term = parse_mul(stack);
    let expr = parse_add_p(stack);
    return Some(ReduceResult::new_box(bin_op_t, term, expr));
}

fn unexpected_token(expected: Token, actual: Token) -> String {
    return format!(
        "Unexpected token. Expected: {:?}. Got: {:?}",
        expected, actual
    );
}

fn parse_add_op(t: Token) -> ReduceResultType {
    return match t {
        Token::Plus => ReduceResultType::Plus,
        Token::Minus => ReduceResultType::Minus,
        _ => panic!(unexpected_token(Token::Plus, t)),
    };
}

fn parse_mul<'a, T>(stack: &mut TokenStack<'a, T>) -> Box<EvalExpression>
    where T: Iterator<Item=Token> + 'a {
    let mut left = parse_term(stack);
    let mut right = parse_mul_p(stack);
    loop {
        match right {
            Some(r) => {
                left = reduce_to_eval_expression(r.op, left, r.term);
                right = r.right;
            }
            None => return left,
        }
    }
}

fn parse_mul_p<'a, T>(stack: &mut TokenStack<'a, T>) -> Option<Box<ReduceResult>>
    where T: Iterator<Item=Token> + 'a {
    match stack.head() {
        Some(Token::Times) | Some(Token::Div) => {}
        _ => return None,
    }
    let t = stack.pop().unwrap();
    let bin_op_t = parse_mul_op(t);
    let term = parse_term(stack);
    let expr = parse_mul_p(stack);
    return Some(ReduceResult::new_box(bin_op_t, term, expr));
}

fn parse_mul_op(t: Token) -> ReduceResultType {
    return match t {
        Token::Times => ReduceResultType::Times,
        Token::Div => ReduceResultType::Div,
        _ => panic!(unexpected_token(Token::Times, t)),
    };
}

fn parse_term<'a, T>(stack: &mut TokenStack<'a, T>) -> Box<EvalExpression>
    where T: Iterator<Item=Token> + 'a {
    let n = next_token(stack);
    return match n {
        Token::Value(value) => EvalExpression::value_box(value),
        Token::LeftParenthesis => {
            let expr = parse_add(stack);
            let n2 = next_token(stack);
            match n2 {
                Token::RightParenthesis => expr,
                t => panic!(unexpected_token(Token::RightParenthesis, t)),
            }
        }
        t => panic!(unexpected_token(Token::Value(0), t)),
    };
}

fn next_token<'a, T>(stack: &mut TokenStack<'a, T>) -> Token
    where T: Iterator<Item=Token> + 'a {
    match stack.pop() {
        Some(t) => return t,
        None => panic!("Unexpected end of token"),
    }
}

pub fn parse<'a, T, L>(lexer: &'a L) -> Box<EvalExpression>
    where T: Iterator<Item=Token> + 'a, L: Lexer<'a, T> {
    let mut stack = TokenStack::new(lexer.tokens());
    let result = parse_add(&mut stack);
    match stack.head() {
        Some(t) => panic!(format!("Unexpected token: {:?}", t)),
        None => return result,
    }
}
