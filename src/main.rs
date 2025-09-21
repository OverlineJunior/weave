#![allow(dead_code, unused_variables)]

use crate::{interpreter::interpret, lexer::tokenize, parser::parser};
use chumsky::prelude::*;

mod lexer;
mod parser;
mod interpreter;

const SOURCE: &str = r#"
	component Bar {
		baz,
		qux,
	}

	var bar = Bar { baz: 1, qux: "a" }

	entity(bar)

	system Foo(bar: Bar) {
		print bar.baz
	}
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
