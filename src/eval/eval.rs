use std::rc::Rc;
use ast::types::*;
use eval::types::*;

pub fn eval<'a>(expr: Expression, stack: &'a mut CallStack) {

    stack.push_frame();

    let out = match expr {
        Expression::Call(symbol, arguments) => {

            // println!("calling: {:?}", symbol);

            if stack.env.borrow().language_functions.contains_key(&symbol) {

                // println!("hey in language functions");

                let env_rc = Rc::clone(&stack.env);
                let env = env_rc.borrow();
                let func = env.language_functions.get(&symbol).as_ref().unwrap().clone();

                func.call((arguments, stack))

            } else {

                // println!("hey before evaluating arguments");

                // evaluate all arguments
                let mut arg_values = Vec::<Value>::new();

                for sub_expr in arguments {
                    // println!("argument {:?}", sub_expr);

                    eval(sub_expr, stack);

                    arg_values.push(stack.pop_return());

                };

                // println!("evaluated arguments");

                if let Some(func) = stack.env.borrow().static_functions.get(&symbol) {
                    // println!("hey calling static function {:?}", symbol);

                    func.call((arg_values,))
                }

                // if the function is dynamic, call it
                else if let Value::Function(ref func) = stack.get_value(&symbol) {
                    // println!("found dynamic function symbol {:?}", symbol);

                    func.call(arg_values)

                }

                else {
                    // println!("uhhhhh");
                    Value::Null }
            }

        },
        Expression::Symbol(ref symbol) => {
            stack.get_value(symbol)
        },
        Expression::Value(value) => value,
        _ => Value::Null
    };

    stack.top().borrow_mut().returns = out;
}
