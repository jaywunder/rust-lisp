use std::collections::HashMap;
use ast::types::*;
use eval::types::*;
use eval;

pub type LanguageFn = Fn(Vec<Expression>, &mut CallStack) -> Value;

pub fn init(scope: &mut HashMap<Symbol, Box<LanguageFn>>) {
    scope.insert(
        String::from("let"),
        box | args, stack | -> Value {
            stack.pop_return();

            for i in (0..args.len()).step_by(2) {
                match args[i] {
                    Expression::Symbol(ref symbol) => {

                        eval::eval::eval(args[i+1].clone(), stack);

                        let value = stack.pop_return().clone();
                        stack.push_value(symbol.clone(), value)
                    }
                    _ => return Value::Null,
                }
            }

            let i = stack.stack.len();

            Value::Null
        }
    );
    
    scope.insert(
        String::from("if"),
        box | args, stack | -> Value {

            let mut eval_else = true;

            for i in (0..args.len() - 2).step_by(2) {
                eval::eval::eval(args[i].clone(), stack);
                if let Value::Boolean(val) = stack.pop_return().clone() {
                    if val {
                        eval::eval::eval(args[i + 1].clone(), stack);
                        eval_else = false;
                        break;
                    }
                }
            }

            if eval_else {
                eval::eval::eval(args[args.len() - 1].clone(), stack);
            }

            stack.pop_return()
        }
    );
    
    scope.insert(
        String::from("do"),
        box | args, stack | -> Value {

            for i in 0..args.len() {
                eval::eval::eval(args[i].clone(), stack);
                let value = stack.pop_return();
                stack.top().returns = value;
            }

            stack.pop_return()
        }
    );
    
    scope.insert(
        String::from("func"),
        box | args, stack | -> Value {

            for i in 0..args.len() {
                eval::eval::eval(args[i].clone(), stack);
                let value = stack.pop_return();
                stack.top().returns = value;
            }

            stack.pop_return()
        }
    );
}
