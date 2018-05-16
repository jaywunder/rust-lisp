#![allow(dead_code)]
use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use eval::cactus::Cactus;
use ast::types::*;
use eval::builtins::{
    static_scope,
    static_functions::{ self, StaticFn },
    language_functions::{ self, LanguageFn }
};

static mut next_id: i32 = 0;

#[derive(Debug)]
pub struct Frame {
    pub id: i32,
    pub scope: HashMap<Symbol, Value>,
    pub returns: Value,
    pub arguments: Vec<Value>,
}

impl Frame {
    pub fn new() -> Frame {

        unsafe {next_id += 1;}

        Frame {
            id: unsafe{next_id - 1},
            scope: HashMap::new(),
            returns: Value::Null,
            arguments: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CallStack {
    pub stack: Cactus<Frame>,
    pub env: Rc<RefCell<Env>>,
}

pub struct Env {
    pub static_scope: HashMap<Symbol, Value>,
    pub static_functions: HashMap<Symbol, Box<StaticFn>>,
    pub language_functions: HashMap<Symbol, Box<LanguageFn>>,
}

impl CallStack {

    pub fn new() -> CallStack {
        CallStack {
            stack: Cactus::new().push(Frame::new()),
            env: Rc::new(RefCell::new(Env::new()))
        }
    }

    pub fn new_from(stack: &mut Cactus<Frame>, env: &mut Rc<RefCell<Env>>) -> CallStack {
        CallStack {
            stack: stack.clone(),
            env: Rc::clone(env),
        }
    }

    pub fn push_value(&mut self, symbol: Symbol, value: Value) {
        // println!("inserting {:?} to be {:?} in frame {:?}", symbol, value, self.stack.len());
        self.stack.peek().map(|frame| frame.borrow_mut().scope.insert(symbol.clone(), value));
        // self.stack.peek().map(|frame| println!("did it insert? {:?}", frame));

        // println!("    {:?}", self.stack);

    }

    pub fn get_value(&self, symbol: &Symbol) -> Value {

        // println!("getting value {:?}", symbol);

        if let Some(value) = self.env.borrow().static_scope.get(symbol) {

            // println!("found in static scope");

            value.clone()

        } else {

            // println!("looking in dynamic scope");

            for i in 0..self.stack.len() {
                // println!("i: {:?} scope: {:?}", i, self.stack[i].borrow().scope);

                if let Some(val) = self.stack[i].borrow().scope.get(symbol) {
                    return val.clone() // TODO: make this not a clone
                }
            }

            Value::Null
        }
    }

    pub fn push_frame(&mut self) {
        self.stack = self.stack.push(Frame::new());
        // println!("PUSHING FRAME {:?} {:?}", self.stack.len() - 1, self.stack.peek().unwrap().borrow());
    }

    pub fn pop_return(&mut self) -> Value {

        self.stack.peek().map(|frame| {
            if let Value::Function(ref func) = frame.borrow().returns {

            }
        });

        // if you're returning a function that was created in this frame, don't drain the scope
        self.stack.peek().map(|frame| { frame.borrow_mut().scope.drain(); });

        if let Some(frame) = self.stack.pop() {

            frame.returns

        } else { Value::Null }
    }

    pub fn top(&mut self) -> &RefCell<Frame> {
        // println!("ABOUT TO UNWRAP TOP {:?}", self.stack.peek().is_some());
        self.stack.peek().unwrap()
    }

}

impl Env {
    pub fn new() -> Env {
        let mut language_functions_map: HashMap<Symbol, Box<LanguageFn>> = HashMap::new();
        language_functions::init(&mut language_functions_map);

        let mut static_functions_map: HashMap<Symbol, Box<StaticFn>> = HashMap::new();
        static_functions::init(&mut static_functions_map);

        let mut static_scope_map: HashMap<Symbol, Value> = HashMap::new();
        static_scope::init(&mut static_scope_map);

        Env {
            static_scope: static_scope_map,
            static_functions: static_functions_map,
            language_functions: language_functions_map,
        }
    }

    // pub fn static_value(&mut self, symbol: Symbol, value: Value) {
    //     self.env.get_mut().static_scope.insert(symbol, value);
    // }
    //
    // pub fn get_static_value(&mut self, symbol: &Symbol) -> Option<&Value> {
    //     self.env.get_mut().static_scope.get(symbol)
    // }
    //
    // pub fn static_function(&mut self, symbol: Symbol, value: Box<Fn(Vec<Value>) -> Value>) {
    //     self.env.get_mut().static_functions.insert(symbol, value);
    // }
    //
    // pub fn get_static_function(&mut self, symbol: &Symbol) -> Option<&Box<Fn(Vec<Value>) -> Value>> {
    //     self.env.get_mut().static_functions.get(symbol)
    // }
    //
    // pub fn language_function(&mut self, symbol: Symbol, value: Box<LanguageFn>) {
    //     self.env.get_mut().language_functions.insert(symbol, value);
    // }
    //
    // pub fn get_language_function(&mut self, symbol: &Symbol) -> Option<&Box<LanguageFn>> {
    //     self.env.get_mut().language_functions.get(symbol)
    // }
    //
    // pub fn has_language_function(&mut self, symbol: &Symbol) -> bool {
    //     self.env.get_mut().language_functions.contains_key(symbol)
    // }
}
impl fmt::Debug for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{{{{Env}}}}") }
}
