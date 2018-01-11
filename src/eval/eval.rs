use ast::types::*;
use eval::types::*;

pub fn eval<'a>(expr: Expression, stack: &'a mut CallStack) {

    stack.push_frame();

    let out = match expr {
        Expression::Call(symbol, arguments) => {

            if let Some(func) = stack.language_functions.remove(&symbol) {

                let symbol_clone = symbol.clone();
                let value = func.call((arguments, stack));

                stack.language_functions.insert(symbol_clone, func);

                value

            } else {

                // evaluate all arguments
                let mut arg_values = Vec::<Value>::new();

                for sub_expr in arguments {

                    eval(sub_expr, stack);

                    arg_values.push(stack.pop_return());

                };

                if let Some(func) = stack.get_static_function(&symbol) {
                    func.call((arg_values,))
                }

                // if the function is dynamic, call it
                // else if let Value::Function(ref func) = stack.get_value(&symbol) {
                //
                //     // func.call(arg_values)
                //     Value::Null // TODO: Change this
                // }

                else { Value::Null }
            }

        },
        Expression::Symbol(ref symbol) => {
            stack.get_value(symbol)
        },
        Expression::Value(value) => value,
        _ => Value::Null
    };

    stack.top().returns = out;

    // Value::Number(42 as f32)
}
