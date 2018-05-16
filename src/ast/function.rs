#![allow(dead_code, unused_imports)]

use std::cell::RefCell;
use ast::types::*;
use eval::{ types::{Frame, CallStack}, cactus::Cactus, eval::eval };

#[derive(Clone)]
pub struct Function {
    pub local_stack: RefCell<CallStack>,
    pub arguments: Vec<Symbol>,
    pub exprs: Vec<Expression>,
}

impl Function {
    pub fn new(local_stack: CallStack, arguments: Vec<Symbol>, exprs: Vec<Expression>) -> Function {
        Function {
            local_stack: RefCell::new(local_stack),
            arguments, exprs
        }
    }

    pub fn call(&self, arguments: Vec<Value>) -> Value {

        // println!("WOWOWOWOWOWOWOW calling that function!");

        let mut local_stack = self.local_stack.borrow_mut();

        local_stack.push_frame();

        for i in 0..self.arguments.len() {
            local_stack.push_value(self.arguments[i].clone(), arguments[i].clone());
        }

        for i in 0..self.exprs.len() {
            eval(self.exprs[i].clone(), &mut local_stack);
            let value = local_stack.pop_return();
            local_stack.top().borrow_mut().returns = value;
        }

        local_stack.pop_return()
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Function) -> bool {
        false // TODO: not this shit
    }
}
