#![allow(unused_imports)]
use std::convert::AsRef;
use std::error::Error;
use std::io::prelude::*;
use std::str::FromStr;

use std::io::BufReader;
use std::fs::File;

use super::env::*;//{Expression, ExpressionStream};
// use super::eval::{eval};
use super::types::*;


pub fn parse(path: &str) -> TokenStream {

    let buf = read_file(path);

    let stream = split(buf);

    return tokenize(stream);
}

fn read_file(path: &str) -> BufReader<File> {
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    reader
}

fn split(reader: BufReader<File>) -> Vec<String> {
    let mut stream: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Err(why) => panic!("couldn't split line: {}", Error::description(&why)),

            Ok(line) => {
                let mut line1 = line.replace("(", " ( ").replace(")", " ) ");
                line1 = line1.replace("'", " ' ");
                line1 = line1.replace("\"", " \" ");

                for string in line1.split_whitespace() {
                    stream.push(string.to_string());
                }
            }
        }
    }

    stream
}

fn tokenize(stream: Vec<String>) -> TokenStream {
    let mut final_stream: TokenStream = Vec::new();

    for token in stream {
        final_stream.push(match token.as_ref() {
            "(" => Type::OpenParen,
            ")" => Type::CloseParen,
            "nil" => Type::Nil,
            // Not going to implement numbers quite yet sorry
            "3" => {
                Type::Number(
                    match f64::from_str(token.as_ref()) {
                        Err(why) => panic!(why),
                        Ok(num) => num
                    }
                )
            }
            _   => Type::Atom(token.to_string()),
        })
    }

    return final_stream
}
