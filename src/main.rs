#![feature(box_syntax, box_patterns, fn_traits, nll, iterator_step_by)]
#![allow(unused_macros)]
#[macro_use]
extern crate nom;
extern crate cactus;

mod ast;
mod eval;

use ast::*;

const DASHES: &'static str = "--------------------------------";

macro_rules! run_program {
    ($program:expr) => {
        let _cyan = r#"\[\033[0;36m\]"#; // TODO: maybe later
        let _clear = r#"\[\033[0m\]"#;

        let mut stack = eval::types::CallStack::new();

        stack.top().borrow_mut().scope.insert(String::from("uhh"), ast::types::Value::Number(42f32));

        let parse_result = debug_expr($program);

        println!("Program:\n{}\n", String::from_utf8($program.to_vec()).unwrap());

        println!("Parse Result:\n{:?}", parse_result);

        if let nom::IResult::Done(_a, b) = parse_result {
            println!("\nEvaluating:");
            eval::eval::eval(b, &mut stack);
            println!("\nReturns:\n{:?}", stack.pop_return());
        }
        // else if let nom::IResult::Error(e) = parse_result {
        //     println!("{:?}", e);
        // }

        println!("{}", DASHES);
    };
}

fn main() {

    println!("{}", DASHES);

    run_program!(b"(plus 2 2)");
    run_program!(b"(print 2 2)");
    // run_program!(b"(print \"null:\" nothing)");
    run_program!(b"(let a 1 b 1)");
    run_program!(b"(let a 1)");
    run_program!(b"(let a 1)");
    // run_program!(b"(let a 1 b 2)");
    // run_program!(br#"(let a 1 b "hey")"#);
    // run_program!(br#"(if true (print "wow it works") (print "fucking dammit"))"#);
    run_program!(br#"(if
        false (print "oh boy no no no")
        true (print "wow it works")
        (print "heckin darn"))"#);
    // run_program!(br#"(if
    // true (plus 1 1)
    // false (print "wow it works")
    // "fucking NICE")"#);
    run_program!(br#"(do
    (print "print 1")
    (print "print 2")
    (plus 1 1)
    (plus 2 2)
    )"#);

    run_program!(br#"(do
        (let a 1)
        (print "printing a")
        (print a)
        (func hello (arg1 arg2) (do
            (print "hello functions!")
        ))

        (do (do (do (hello 1 2))))
        (do (do (do (hello 1 2))))
    )"#);

    // THE TOUGH-Y
    // run_program!(br#"(do
    //     (let a 1)
    //     (print "printing a")
    //     (print a)
    //
    //     (func returnfunc (arg1) (print "returnfunc"))
    //
    //     (func hello (arg1 arg2) (do
    //         (set a (plus a 1))
    //         (print "hello functions!")
    //
    //         (func inner (n) (do
    //             (set a (plus a n))
    //         ))
    //
    //         returnfunc
    //     ))
    //
    //     (hello 1 2)
    //     (print a)
    //     (do (do (do (hello 1 2))))
    //     (print a)
    //     (hello 1 2)
    // )"#);

    // run_program!(br#"(do
    //     (let b 1)
    //     (print b)
    //     (func closure (arg) (do
    //         (let a 0)
    //         (func incA (n) (do
    //             (set a (plus a n))
    //         ))
    //         incA
    //     ))
    //     (print b)
    //     (let thing (closure 1))
    //     (print "a = " (thing 1))
    //     (print "a = " (thing 2))
    //     (print "a = " (thing 3))
    //     (print "a = " (thing 4))
    // )"#);

    // run_program!(br#"
    // (do
    //     (defun closure () (do
    //         (let a 0)
    //         (defun incA (n) (let a (do
    //             (plus a n)
    //             a
    //         ))
    //     ))
    //
    //     (let thing (closure))
    //     (print "a = " (thing 1))
    //     (print "a = " (thing 2))
    //     (print "a = " (thing 3))
    //     (print "a = " (thing 4))
    // )
    // "#);

    // let thing = expr(b"(hey woahs woahs (woah wow.wow) 42 3.14 null \"hey there\" what.what)");
    // let thing2 = program(b"(hey woahs woahs (woah wow.wow) 42 3.14 null \"hey there\" what.what)(mhm yeah)");
    // let thing3 = expr(b"(plus 1 1)");
    // let thing4 = expr(b"(let a 1)");
    //
    // let mut stack = eval::types::CallStack::new();
    //
    // // println!("\n{:?}\n", thing);
    // // println!("\n{:?}\n", thing2);
    // println!("\n{:?}\n", thing3);
    //
    // if let nom::IResult::Done(a, b) = thing3 {
    //     eval::eval::eval(b, &mut stack);
    //     println!("\n{:?}\n", stack.pop_returns());
    // }
    //
    // println!("\n{:?}\n", thing4);
    //
    // if let nom::IResult::Done(a, b) = thing4 {
    //     eval::eval::eval(b, &mut stack);
    //     println!("\n{:?}\n", stack.pop_returns());
    // }

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

#[test]
fn test_cactus() {
    let mut main = eval::cactus::Cactus::new();
    use std::cell::RefCell;
    assert!(main.is_empty());

    main = main.push(0);
    assert_eq!(main.pop().unwrap(), 0);

    main = main.push(0);
    main = main.push(1);
    main = main.push(2);

    assert_eq!(main.len(), 3);

    let mut other0 = main.push(3);
    let mut other1 = main.push(10);

    println!("{:?}", main[0]);
    println!("{:?}", main[1]);
    println!("{:?}", main[2]);
    println!("{:?}", other0.peek());
    println!("{:?}", other1.peek());

    other0.pop();
    // other0.pop();
    // other0.pop();

    println!("{:?}", main[2]);
}
