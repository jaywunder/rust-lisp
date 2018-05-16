#![allow(unused_imports, unused_must_use, dead_code)]
use nom::{IResult, digit, space, alpha, alphanumeric};

use std::str;
use std::str::FromStr;

use std::fmt;
use std::fmt::Write;
use function::Function;

pub type Symbol = String;
pub type Program = Vec<Expression>;

#[derive(Clone, PartialEq)]
pub enum Value {
    Number(f32),
    String(String),
    Function(Function),
    Boolean(bool),
    Null,
}

#[derive(Clone)]
pub enum Expression {
    // Block(Vec<Expression>),
    Symbol(Symbol),
    Call(Symbol, Vec<Expression>),
    DotAccess(Vec<Symbol>),
    Value(Value),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expression::*;

        match self {
            &Call(ref s, ref v) => {
                let mut args = String::new();
                for arg in v {
                    write!(&mut args, "\n{}", arg);
                }
                args = args.replace("\n", "\n  ");

                if args.len() > 0 { write!(f, "Call({}{}\n)", s, args) }
                else { write!(f, "Call({})", s) }
            },
            &Symbol(ref s) => write!(f, "symbol[{}]", s),
            &DotAccess(ref v) => write!(f, "DotAccess{:?}", v),
            &Value(ref x) => write!(f, "{}", x)
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self) }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Value::*;

        match self {
            &Number(ref n) => write!(f, "number[{}]", n),
            &String(ref s) => write!(f, "string[{}]", s),
            &Function(ref _s) => write!(f, "function[something]"),
            &Boolean(ref b) => write!(f, "boolean[{}]", b),
            &Null => write!(f, "[null]"),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self) }
}
