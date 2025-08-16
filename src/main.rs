use crate::{analyzer::analyze, lexer::tokenize, parser::parser, type_env::TypeEnv};
use chumsky::prelude::*;

mod lexer;
mod expr;
mod stmt;
mod parser;
mod r#type;
mod type_env;
mod analyzer;
mod semantic_error;
mod error;

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
		baz: Int,
		qux: String,
	}

	Bar {
		baz: 1,
		qux: "b",
	}
"#;

fn main() {
	let spanned_tokens = tokenize(SOURCE.to_string()).expect("Failed to tokenize source");
	let tokens = spanned_tokens.into_iter().map(|(_, t, _)| t).collect::<Vec<_>>();
	let ast = parser().parse(tokens.as_slice()).unwrap();
	let mut type_env = TypeEnv::new();
	let decorated_ast = analyze(&ast, &mut type_env).expect("Failed to analyze AST");
	println!("AST:\n{:#?}\n", ast);
	println!("Decorated AST:\n{:#?}\n", decorated_ast);
	println!("Type Environment:\n{:#?}\n", type_env);
}
