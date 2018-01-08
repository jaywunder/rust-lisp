#![feature(box_syntax, box_patterns, fn_traits)]
#[macro_use]
extern crate nom;

mod ast;
mod eval;

use std::rc::Rc;
// use std::collection

use ast::*;

fn main() {
    let thing = expr(b"(hey woahs woahs (woah wow.wow) 42 3.14 null \"hey there\" what.what)");
    let thing2 = program(b"(hey woahs woahs (woah wow.wow) 42 3.14 null \"hey there\" what.what)(mhm yeah)");
    let thing3 = expr(b"(plus 1 1)");

    let mut stack = eval::CallStack::new();
    stack.static_function(
        String::from("plus"),
        box |args| -> types::Value {
            use types::Value::*;

            println!("IN THE CALL {:?}", args);

            if args.len() < 2 { return Null }

            if let Number(ref a) = args[0] {
                if let Number(ref b) = args[1] {
                    Number(a + b)
                } else { Null }
            } else { Null }
        }
    );

    // println!("\n{:?}\n", thing);
    // println!("\n{:?}\n", thing2);
    println!("\n{:?}\n", thing3);

    if let nom::IResult::Done(a, b) = thing3 {
        // match eval::eval(b, &stack) {
        //     eval::Return::Raw(v) => println!("\n{:?}\n", v),
        //     eval::Return::Ref(ref v) => println!("\n{:?}\n", v),
        // };
        // println!("\n{:?}\n", eval::eval(&b, &stack));
        eval::eval(b, &mut stack);
        println!("\n{:?}\n", stack.top().returns);
    }

    // if let IResult::Done(a, b) = thing {
    //     println!("{:?}", str::from_utf8(a).unwrap());
    //     println!("{}", b);
    //     // println!("{:?}", String::from_utf8(b.to_vec()).unwrap());
    //
    //     // println!("{}", b);
    // } else if let IResult::Error(e) = thing {
    //     println!("whoops, {}", e.description());
    // }
    //
    // assert_eq!(),  IResult::Done(&b[])
}
