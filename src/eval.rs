use std::collections::HashMap;
use std::rc::Rc;

use ast::types::*;

pub fn eval<'a>(expr: Expression, stack: &'a mut CallStack) {
    stack.push_frame();

    let out = match expr {
        Expression::Call(symbol, arguments) => {

            // evaluate all arguments
            let mut arg_values = Vec::<Value>::new();

            for sub_expr in arguments {

                eval(sub_expr, stack);

                let mut returns = stack.pop_returns().unwrap();

                for value in returns {
                    arg_values.push(value);
                }
            };

            // if the function is static, call it
            if let Some(func) = stack.get_static_function(&symbol) {
                func.call((arg_values,))
            }

            // if the function is dynamic, call it
            // else if let Value::Function(ref func) = stack.get_value(&symbol) {
            //
            //     // func.call(arg_values)
            //     Value::Null // TODO: Change this
            // }

            else { Value::Null }
        },
        // Expression::Symbol(ref symbol) => {
        //     stack.get_value(symbol)
        // },
        Expression::Value(value) => value,
        _ => Value::Null
    };

    stack.top().returns.push(out);

    // Value::Number(42 as f32)
}

pub struct CallStack {
    stack: Vec<Frame>,
    static_scope: HashMap<Symbol, Value>,
    static_functions: HashMap<Symbol, Box<Fn(Vec<Value>) -> Value>>,
}

impl CallStack {

    pub fn new() -> CallStack {
        CallStack {
            stack: vec![Frame::new()],
            static_scope: HashMap::new(),
            static_functions: HashMap::new(),
        }
    }

    pub fn push_value(&mut self, symbol: Symbol, value: Value) {
        if self.stack.len() > 0 {
            let i = self.stack.len() - 1;
            self.stack[i].scope.insert(symbol, value);
        }
    }

    pub fn get_value(&self, symbol: &Symbol) -> Value {

        if let Some(value) = self.static_scope.get(symbol) {

            value.clone()

        } else {
            let mut i = self.stack.len() - 1;
            while i >= 0 {
                if let Some(val) = self.stack[i].scope.get(symbol) {
                    return val.clone();
                }
                else { i -= 1 }
            }

            Value::Null
        }

    }

    pub fn push_frame(&mut self) -> &mut Frame {
        self.stack.push(Frame::new());

        self.top()
    }

    pub fn pop_returns(&mut self) -> Option<Vec<Value>> {
        if let Some(frame) = self.stack.pop() {
            Some(frame.returns)
        } else { None }
    }

    pub fn top(&mut self) -> &mut Frame {
        let len = self.stack.len();
        &mut self.stack[len - 1]
    }

    pub fn static_value(&mut self, symbol: Symbol, value: Value) {
        self.static_scope.insert(symbol, value);
    }

    pub fn static_function(&mut self, symbol: Symbol, value: Box<Fn(Vec<Value>) -> Value>) {
        self.static_functions.insert(symbol, value);
    }

    pub fn get_static_function(&mut self, symbol: &Symbol) -> Option<&Box<Fn(Vec<Value>) -> Value>> {
        self.static_functions.get(symbol)
    }

    // pub fn clear_returns(&mut self) {
    //     self.top().returns.clear()
    // }
    //
    // pub fn push_return(&mut self, value: Value) {
    //     self.top().returns.push(value)
    // }
    //
    // pub fn pop_return(&mut self) -> Option<Value> {
    //     self.top().returns.pop()
    // }
    //
    // pub fn set_returns(&mut self, returns: Vec<Value>) {
    //     let len = self.stack.len();
    //     self.stack[len - 1].returns = returns;
    // }
    //
    // pub fn get_returns(&mut self) -> &mut Vec<Value> {
    //     &mut self.top().returns
    // }

}

pub struct Frame {
    pub scope: HashMap<Symbol, Value>,
    pub returns: Vec<Value>,
    pub arguments: Vec<Value>,
}

impl Frame {

    pub fn new() -> Frame {
        Frame {
            scope: HashMap::new(),
            returns: Vec::new(),
            arguments: Vec::new(),
        }
    }

}
