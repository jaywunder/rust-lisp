#![allow(dead_code)]
use std::collections::HashMap;
use super::types::*;

pub struct Env {
    bindings: HashMap<String, Expression>,
}

impl Env {
    fn new() -> Env {
        Env { bindings: HashMap::new() }
    }

    fn set() {

    }
}
