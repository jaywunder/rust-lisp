#![allow(dead_code)]
use std::collections::HashMap;
use super::types::*;

pub struct Env {
    pub expression_stream: Type,
    pub bindings: HashMap<Atom, ExpressionStream>,
    pub intrinsics: HashMap<Atom, ExpressionStream>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            expression_stream: Type::Expression(Vec::new()),
            bindings: HashMap::new(),
            intrinsics: HashMap::new(),
        }
    }

    pub fn set(&mut self, atom: Atom, stream: ExpressionStream) {

        self.bindings.insert(atom, stream);

    }

    pub fn get(&mut self, atom: &Atom) -> Option<&ExpressionStream> {

        self.bindings.get(atom)

    }

}
