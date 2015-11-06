#![allow(dead_code)]
use std::collections::HashMap;
use super::types::*;

pub struct Env<'a> {
    bindings: HashMap<Atom, &'a mut ExpressionStream>,
    intrinsics: HashMap<Atom, ExpressionStream>,
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            bindings: HashMap::new(),
            intrinsics: HashMap::new(),
        }
    }

    fn set(&mut self, atom: Atom, stream: &'a mut ExpressionStream) {

        self.bindings.insert(atom, stream);

    }

    fn get(&mut self, atom: &Atom) -> Option<&&mut ExpressionStream> {

        self.bindings.get(atom)

    }
}
