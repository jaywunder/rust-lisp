#![feature(box_syntax, box_patterns, fn_traits, nll, iterator_step_by)]
#![allow(unused_macros, unused_imports)]
#[macro_use]
extern crate nom;
extern crate cactus;
extern crate colored;

mod ast;
mod eval;
#[macro_use]
mod macros;

use colored::*;
use ast::*;
use types::Value;

const DASHES: &'static str = "--------------------------------";

fn main() {

    println!("{}", DASHES);

    // run_program!(b"(plus 2 2)");
    // run_program!(b"(print 2 2)");
    // run_program!(b"(print \"null:\" nothing)");
    // run_program!(b"(let a 1 b 1)");
    // run_program!(b"(let a 1)");
    // run_program!(b"(let a 1)");
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
}


mod tests {
    use super::ast::*;
    use super::eval;
    use super::ast::types::Value::*;

    lisp_tests! {
        addition_pos: (b"(plus 1 1)", Number(2 as f32)),
        // addition_neg: (b"(plus -1 -1)", Number(-2 as f32)),
        if_1st: (br#"(if true 1 false 2 3)"#, Number(1 as f32)),
        if_2nd: (br#"(if false 1 true 2 3)"#, Number(2 as f32)),
        if_else: (br#"(if false 1 false 2 3)"#, Number(3 as f32)),
    }

    
}


// #[test]
// fn test_cactus() {
//     let mut main = eval::cactus::Cactus::new();
//     use std::cell::RefCell;
//     assert!(main.is_empty());
//
//     main = main.push(0);
//     assert_eq!(main.pop().unwrap(), 0);
//
//     main = main.push(0);
//     main = main.push(1);
//     main = main.push(2);
//
//     assert_eq!(main.len(), 3);
//
//     let mut other0 = main.push(3);
//     let mut other1 = main.push(10);
//
//     println!("{:?}", main[0]);
//     println!("{:?}", main[1]);
//     println!("{:?}", main[2]);
//     println!("{:?}", other0.peek());
//     println!("{:?}", other1.peek());
//
//     other0.pop();
//     // other0.pop();
//     // other0.pop();
//
//     println!("{:?}", main[2]);
// }
