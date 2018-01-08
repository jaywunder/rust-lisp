#![allow(dead_code)]

use ast::types::*;
// use eval::Frame;

#[derive(Clone)]
pub enum ValueOrigin {
    Static(String),  // rust fn
    Dynamic(), // lisp fn
}

#[derive(Clone)]
pub struct Function {
    // frame: Frame,
    // arguments: Vec<Argument>,
    function: ValueOrigin,
}

impl Function {
    fn new(
        // frame: Frame,
        // arguments: Vec<Argument>,
        function: ValueOrigin
    ) -> Function {
        Function {
            // frame,
            // arguments,
            function,
        }
    }

    // pub fn call(&self, arguments: Vec<Value>) -> Value {
    // // pub fn call(&self, this: &mut Value) -> Value {
    //     match &self.function {
    //         ValueOrigin::Static(ref func) => func(arguments)
    //     }
    //
    //
    //
    //     // Value::Number(42 as f32)
    // }
}

pub struct Argument {
    pub symbol: Symbol
}
