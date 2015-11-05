#![allow(dead_code)]
// use std::collections::HashMap;
use std::error::Error;

pub type Token = Type;
pub type TokenStream = Vec<Token>;

pub type ExpressionStream = Vec<Expression>;
pub type EvalResult = Result<Expression, Error>;

pub enum Type {
    OpenParen,
    CloseParen,
    Nil,
    Number(f64),
    Atom(String),
    Func(ExpressionStream),
}

pub struct Expression {
    pub func: Type,
    pub args: ExpressionStream,
}

impl Expression {
    pub fn new(func: Type, args: ExpressionStream) -> Expression {
        Expression {
            func: func,
            args: args,
        }
    }
}
