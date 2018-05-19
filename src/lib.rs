#![feature(box_syntax, box_patterns, fn_traits, nll, iterator_step_by)]
#![allow(unused_macros, unused_imports)]

#[macro_use]
extern crate nom;
extern crate cactus;
extern crate colored;

mod ast;
mod eval;
#[macro_use]
mod macros;

use colored::*;
use ast::*;
pub use types::Value;

use eval::eval::eval;

pub fn run_program(program: String) -> Value {
    let mut stack = eval::types::CallStack::new();
    let parse_result = debug_expr(program.as_bytes());

    if let Ok((_a, b)) = parse_result {
        eval::eval::eval(b, &mut stack);
        return stack.pop_return()
    }

    Value::Null
}
