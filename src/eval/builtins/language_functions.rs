use std::collections::HashMap;
use ast::function::Function;
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

                        let value = stack.pop_return();
                        stack.push_value(symbol.clone(), value);
                        // stack.top().borrow_mut().scope.insert(symbol.clone(), value);
                    }
                    _ => return Value::Null,
                }
            }

            stack.push_frame();

            Value::Null
        }
    );

    scope.insert(
        String::from("set"),
        box | args, stack | -> Value {
            stack.pop_return();

            for i in (0..args.len()).step_by(2) {
                match args[i] {
                    Expression::Symbol(ref symbol) => {

                        eval::eval::eval(args[i+1].clone(), stack);
                        let value = stack.pop_return();

                        for i in 0..stack.stack.len() {
                            if stack.stack[i].borrow().scope.contains_key(symbol) {
                                stack.stack[i].borrow_mut().scope.insert(symbol.clone(), value);
                                break;
                            }
                        }
                    }
                    _ => return Value::Null,
                }
            }

            stack.push_frame();

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
            use eval::eval::eval;

            let mut value = Value::Null;
            for i in 0..args.len() {
                eval(args[i].clone(), stack);
                value = stack.pop_return();
                // stack.top().borrow_mut().returns = value;
            }

            value
        }
    );

    scope.insert(
        String::from("func"),
        box | args, stack | -> Value {

            let mut func_body = args.clone();
            stack.pop_return();

            if let Expression::Symbol(symbol) = func_body.remove(0) {

                if let Expression::Call(arg0, other_args) = func_body.remove(0) {
                    let mut other_args = other_args.clone();
                    other_args.insert(0, Expression::Symbol(arg0));

                    let arguments: Vec<Symbol> = other_args
                        .into_iter()
                        .map(|expr| match expr {
                            Expression::Symbol(symbol) => symbol,
                            _ => String::from("nope"),
                        })
                        .collect();

                    stack.push_value(symbol, Value::Function(
                        Function::new(
                            stack.clone(),
                            arguments,
                            func_body
                        )
                    ));
                }
            }

            stack.push_frame(); // TODO: push value to frames below top

            Value::Null
        }
    );
}
