use crate::lexer::tokenize;

mod value;
mod token;
mod lexing_error;
mod lexer;

const SOURCE: &str = r#"
	system Foo(bar: Bar) {
		print(bar.baz, bar.qux)
	}

	component Bar {
		baz: Int,
		qux: String,
	}

	entity(Bar { baz: 1, qux: "a" })
"#;

fn main() {
	tokenize(SOURCE.to_string())
		.expect("Failed to tokenize source")
		.iter()
		.for_each(|(line, token, _)| {
			println!("Line {}: {}", line, token);
		});
}
