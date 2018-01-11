use std::collections::HashMap;
use ast::types::*;
use eval::types::*;

pub type StaticFn = Fn(Vec<Value>) -> Value;

pub fn init(scope: &mut HashMap<Symbol, Box<StaticFn>>) {
    scope.insert(
        String::from("plus"),
        box | values | -> Value {

            use types::Value::*;

            let mut acc: f32 = 0.0;
            for value in values {
                if let Number(ref a) = value {
                    acc += a;
                } else { return Null }
            }

            Number(acc)
        }
    );

    scope.insert(
        String::from("print"),
        box | values | -> Value {

            for value in values {
                print!("{} ", value);
            }
            println!("");

            Value::Null
        }
    );
}
