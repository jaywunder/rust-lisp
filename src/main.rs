#![feature(box_syntax, box_patterns, fn_traits, nll, use_nested_groups, iterator_step_by)]
#![allow(unused_macros)]
#[macro_use]
extern crate nom;

mod ast;
mod eval;

use ast::*;

const DASHES: &'static str = "--------------------------------";

macro_rules! run_program {
    ($program:expr) => {
        let _cyan = r#"\[\033[0;36m\]"#; // TODO: maybe later
        let _clear = r#"\[\033[0m\]"#;

        let mut stack = eval::types::CallStack::new();

        let parse_result = expr($program);

        println!("Program:\n{}\n", String::from_utf8($program.to_vec()).unwrap());

        println!("Parse Result:\n{:?}", parse_result);

        if let nom::IResult::Done(_a, b) = parse_result {
            println!("\nEvaluating:");
            eval::eval::eval(b, &mut stack);
            println!("\nReturns:\n{:?}", stack.pop_return());
        }

        println!("{}", DASHES);
    };
}


fn main() {

    println!("{}", DASHES);

    run_program!(b"(plus 2 2)");
    run_program!(b"(print 2 2)");
    // run_program!(b"(print \"null:\" nothing)");
    // run_program!(b"(let a 1)");
    // run_program!(b"(let a 1 b 2)");
    // run_program!(br#"(let a 1 b "hey")"#);
    // run_program!(br#"(if true (print "wow it works") (print "fucking dammit"))"#);
    run_program!(br#"(if
        (== 1 2) (print "oh boy no no no")
        (== 1 1) (print "wow it works")
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
    
    run_program!(br#"
    (do
        (defun closure () (do
            (let a 0)
            (defun incA (n) (let a (do
                (plus a n)
                a
            ))
        ))

        (let thing (closure))
        (print "a = " (thing 1))
        (print "a = " (thing 2))
        (print "a = " (thing 3))
        (print "a = " (thing 4))
    )
    "#);

    /*
    global
    |         | thing
    closure   
    | a incA
    incA
    n

    
    */


    // run_program!(b"(switch () (case () ())) ");








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
