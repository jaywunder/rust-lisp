#![allow(unused_imports, unused_must_use)]
use nom::{IResult, digit, space, alpha, alphanumeric};

use std::str;
use std::str::FromStr;

use std::fmt;
use std::fmt::Write;

use types::*;

named!(pub program<Program>, fold_many0!(ws!(expr), Vec::new(), |mut acc: Program, item: Expression| {
    acc.push(item); acc
}));

named!(pub expr<Expression>,
    alt!(
        do_parse!(
            n: tag!("null") >>
            (Expression::Value(Value::Null))
        ) |
        do_parse!(
            n: number >>
            (Expression::Value(Value::Number(n)))
        ) |
        // do_parse!(
        //     s: function >>
        //     (Expression::Value(Value::Function(s)))
        // ) |
        do_parse!(
            s: string >>
            (Expression::Value(Value::String(s)))
        ) |
        do_parse!(
            v: dot_access >>
            (Expression::DotAccess(v))
        ) |
        do_parse!(
            s: symbol >>
            (Expression::Symbol(s))
        ) |
        do_parse!(
            c: function_call >>
            (Expression::Call(c.0, c.1))
        )
    )
);

named!(function_call<(Symbol, Vec<Expression>)>,do_parse!(
    tag!(&['(' as u8][..]) >>
    func: ws!(symbol) >>
    args: ws!(arguments) >>
    tag!(&[')' as u8][..]) >>
    ((func, args))
));

named!(arguments<Vec<Expression>>,
    fold_many0!(ws!(expr), Vec::new(), |mut acc: Vec<Expression>, item: Expression| {
        acc.push(item); acc
    })
);

named!(dot_access<Vec<Symbol>>, do_parse!(
    init: symbol >>
    all: fold_many1!(ws!(pair!(char!('.'), symbol)), vec![init], |mut acc: Vec<Symbol>, item: (char, Symbol)| {
        acc.push(item.1); acc
    }) >>
    (all)
));

// named!(function<String>,
//     do_parse!(
//         tag!("#'") >>
//         sym: symbol >>
//         (sym)
//     )
// );

named!(symbol<Symbol>, do_parse!(
    sym: map!(
        alphanumeric,
        |a: &[u8]| String::from_utf8(a.to_vec()).unwrap()
    ) >>
    (sym)
));

named!(number<f32>, map!(
    alt!(
        recognize!(delimited!(
            digit,
            char!('.'),
            digit
        )) |
        digit
    ),
    |a| FromStr::from_str(str::from_utf8(a).unwrap()).unwrap()
));

named!(string<String>, map!(
    delimited!(
        char!('"'),
        is_not!("\""),
        char!('"')
    ),
    |a| String::from_utf8(a.to_vec()).unwrap()
));
