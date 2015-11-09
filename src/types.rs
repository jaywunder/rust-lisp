#![allow(dead_code)]

use std::fmt;


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
    Expression(ExpressionStream),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #![allow(unused_variables)]
        return match self {
            &Type::OpenParen => write!(f, "("),
            &Type::CloseParen => write!(f, ")"),
            &Type::Nil => write!(f, "nil"),
            &Type::Number(val) => write!(f, "{}", val),
            &Type::Atom(ref val) => write!(f, "{}", val),
            &Type::Expression(ref vec) => write!(f, "<{}>", vec[0])
        }
    }
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
