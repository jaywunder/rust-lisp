use std::collections::HashMap;
use ast::types::*;
use eval::types::*;

pub fn init(scope: &mut HashMap<Symbol, Value>) {
    scope.insert( String::from("nothing"), Value::Null );
}
