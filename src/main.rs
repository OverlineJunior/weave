#![allow(dead_code, unused_variables)]

use crate::{interpreter::interpret, lexer::tokenize, parser::parser};
use chumsky::prelude::*;

mod lexer;
mod parser;
mod interpreter;

const SOURCE: &str = r#"
	component A { f1, f2 }
	var a = A { f1: 42, f2: "hello" }
	entity(a)

	component B { g1 }
	var b = B { g1: "world" }
	entity(b)
"#;

fn main() {
    let spanned_tokens = tokenize(SOURCE.to_string()).expect("Failed to tokenize source");
    let tokens = spanned_tokens
        .into_iter()
        .map(|(_, t, _)| t)
        .collect::<Vec<_>>();
    let ast = parser().parse(tokens.as_slice()).unwrap();
    println!("AST:\n{:#?}\n", ast);
	interpret(&ast).expect("Failed to interpret AST");
}
