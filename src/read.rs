#![allow(unused_imports, unused_variables, dead_code, unused_mut)]
use std::convert::AsRef;
use std::error::Error;
use std::io::prelude::*;
use std::str::FromStr;
use std::ptr;

use std::io::BufReader;
use std::fs::File;
use std::iter::Iterator;

use super::env::*;
use super::types::*;


pub fn parse(path: &str) -> Env {

    let buf = read_file(path);

    let stream = split(buf);

    let tokens = tokenize(stream);

    return into_env(tokens);
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

fn into_env(stream: TokenStream) -> Env {

    let mut env: Env = Env::new();

    return env
}

fn parse_atom (token: &Type) -> &Atom {
    match token {
        &Type::Atom(ref atom) => {
            return atom;
        },
        _ => {
            panic!("{} is not an atom", token)
        }
    }
}

fn parse_expression(env: &mut Env, stream: &TokenStream, i: usize) -> Type {

    for mut i in 0..stream.len() {

        if let Some(token) = stream.get(i) {
            match token {
                &Type::OpenParen => {

                    let key: &Atom = if let Some(atom) = stream.get(i + 1) {
                        parse_atom(atom)
                    } else {
                        panic!("got unexpected end of file");
                    };

                    let value: ExpressionStream = if let Some(atom) = stream.get(i) {
                        Vec::new()
                    } else {
                        panic!("got unexpected end of file")
                    };

                    env.set(key.clone(), value);
                }
                _ => {},
            }

        } else {
            panic!("wtf dude?!?! the program ended unexpectedly.");
        };
    }

    Type::Expression {
        func: "bleh bleh".to_string(),
        args: Vec::new()
    }

}
