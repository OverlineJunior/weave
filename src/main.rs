use crate::{lexer::tokenize, parser::parser};
use chumsky::prelude::*;

mod error;
mod lexer;
mod parser;

// const SOURCE: &str = r#"
// 	system Foo(bar: Bar) {
// 		print(bar.baz, bar.qux)
// 	}

// 	component Bar {
// 		baz: Int,
// 		qux: String,
// 	}

// 	entity(Bar { baz: 1, qux: "a" })
// "#;

const SOURCE: &str = r#"
	component Bar {
		baz,
		qux,
	}

	Bar {
		baz: 1,
		qux: "b",
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
}
