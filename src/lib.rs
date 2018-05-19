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

use eval::eval::eval;
