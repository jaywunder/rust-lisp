#![allow(dead_code)]
// use std::collections::HashMap;

// extern crate core;

// use std::error::Error;
// use std::hash::Hash;

pub type Atom = String;

pub type Token = Type;
pub type TokenStream = Vec<Token>;

pub type ExpressionStream = Vec<Type>;

pub enum Type {
    OpenParen,
    CloseParen,
    Nil,
    Number(f64),
    Atom(Atom),
    Expression {
        func: Atom,
        args: ExpressionStream,
    },
}

// impl PartialEq for Type {
//     fn eq(&self, other: &Type) -> bool {
//         match (self, other) {
//             (&Type::Nil, &Type::Nil) => true,
//             (&Type::OpenParen, &Type::OpenParen) => true,
//             (&Type::CloseParen, &Type::CloseParen) => true,
//             (&Type::Number(ref i), &Type::Number(ref j)) => {
//                 i == j
//             },
//
//             (&Type::Atom(ref i), &Type::Atom(ref j)) => {
//                 i == j
//             },
//
//             (&Type::Expression(ref f0, ref args0), &Type::Expression(ref f1, ref args1)) => {
//
//             },
//             _ => false,
//         }
//     }
//
//     fn ne(&self, other: &Type) -> bool {
//         match (self, other) {
//             (&Type::Nil, &Type::Nil) => false,
//             (&Type::OpenParen, &Type::OpenParen) => false,
//             (&Type::CloseParen, &Type::CloseParen) => false,
//             (&Type::Number(ref i), &Type::Number(ref j)) => i != j,
//             (&Type::Atom(ref i), &Type::Atom(ref j)) => i != j,
//             // (&Type::Func(ref i), &Type::Func(ref j)) => i.eq(j),
//             _ => true,
//         }
//     }
// }
//
// impl Eq for Type {}
