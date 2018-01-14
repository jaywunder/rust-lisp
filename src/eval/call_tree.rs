use cactus::Cactus;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use ast::types::*;
use eval::builtins::{
    static_scope,
    static_functions::{ self, StaticFn },
    language_functions::{ self, LanguageFn }
};

// #[derive(Clone)]
// pub struct Cactus<T> {
//     value: T,
//     parent: Option<Rc<RefCell<Cactus<T>>>>,
//     len: usize,
// }

// impl<T> Cactus<T> {

//     pub fn new(value: T) -> Cactus<T> {
//         Cactus {
//             value,
//             parent: None,
//             len: 1,
//         }
//     }

//     pub fn child(&mut self, value: T) -> Cactus<T> {
//         Cactus {
//             value,
//             parent: Some(self.clone()),
//             len: self.len + 1,
//         }
//     }

//     pub fn value(&mut self) -> &mut T {
//         &mut self.value
//     }



// }

// impl<T> Iterator for Cactus<T> {
//     type Item = RefMut<Cactus<T>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(t) = self.parent {
//             return t.borrow_mut()
//         } else { None }
//     }
// }











pub struct Frame {
    pub scope: HashMap<Symbol, Value>,
    pub returns: Value,
    pub arguments: Vec<Value>,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            scope: HashMap::new(),
            returns: Value::Null,
            arguments: Vec::new(),
        }
    }
}

pub struct CallTree {
    pub stack: Cactus<RefCell<Frame>>,
    pub static_scope: HashMap<Symbol, Value>,
    pub static_functions: HashMap<Symbol, Box<StaticFn>>,
    pub language_functions: HashMap<Symbol, Box<LanguageFn>>,
}

impl CallTree {

    pub fn new() -> CallTree {
        let mut language_functions_map: HashMap<Symbol, Box<LanguageFn>> = HashMap::new();
        language_functions::init(&mut language_functions_map);

        let mut static_functions_map: HashMap<Symbol, Box<StaticFn>> = HashMap::new();
        static_functions::init(&mut static_functions_map);

        let mut static_scope_map: HashMap<Symbol, Value> = HashMap::new();
        static_scope::init(&mut static_scope_map);

        CallTree {
            stack: Cactus::new().child(RefCell::new(Frame::new())),
            static_scope: HashMap::new(),
            static_functions: static_functions_map,
            language_functions: language_functions_map,
        }
    }

    pub fn push_value(&mut self, symbol: Symbol, value: Value) {
        if self.stack.len() > 0 {
            self.stack.val().unwrap().borrow_mut().scope.insert(symbol, value);
        }
    }

    pub fn get_value(&self, symbol: &Symbol) -> Value {

        if let Some(value) = self.static_scope.get(symbol) {

            value.clone()

        } else {

            let mut returns = Value::Null;

            for frame in self.stack.vals() {
                if let Some(value) = frame.borrow_mut().scope.get(symbol) {
                    returns = value.clone();
                    break;
                }
            }

            returns
        }
    }

    pub fn push_frame(&mut self) {
        self.stack = self.stack.child(RefCell::new(Frame::new()));
    }

    pub fn pop_return(&mut self) -> Value {
        if let Some(frame) = self.stack.val().clone() {
            self.stack = self.stack.parent().unwrap();
            frame.borrow_mut().returns
        } else { Value::Null }
    }

    pub fn pop_frame(&mut self) -> Cactus<RefCell<Frame>> {
        let out = self.stack;
        self.stack = self.stack.parent().unwrap();
        out
    }

    pub fn top(&mut self) -> &mut Frame {
        self.stack.val().unwrap().get_mut()
    }

    pub fn static_value(&mut self, symbol: Symbol, value: Value) {
        self.static_scope.insert(symbol, value);
    }

    pub fn get_static_value(&mut self, symbol: &Symbol) -> Option<&Value> {
        self.static_scope.get(symbol)
    }

    pub fn static_function(&mut self, symbol: Symbol, value: Box<Fn(Vec<Value>) -> Value>) {
        self.static_functions.insert(symbol, value);
    }

    pub fn get_static_function(&mut self, symbol: &Symbol) -> Option<&Box<Fn(Vec<Value>) -> Value>> {
        self.static_functions.get(symbol)
    }

    pub fn language_function(&mut self, symbol: Symbol, value: Box<LanguageFn>) {
        self.language_functions.insert(symbol, value);
    }

    pub fn get_language_function(&mut self, symbol: &Symbol) -> Option<&Box<LanguageFn>> {
        self.language_functions.get(symbol)
    }

    pub fn has_language_function(&mut self, symbol: &Symbol) -> bool {
        self.language_functions.contains_key(symbol)
    }

}
