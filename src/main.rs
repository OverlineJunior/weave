use crate::lexer::tokenize;

mod value;
mod token;
mod lexing_error;
mod lexer;

const SOURCE: &str = r#"
	55 "foo bar" 123
	"This is a test string"
"#;

fn main() {
	tokenize(SOURCE.to_string())
		.expect("Failed to tokenize source")
		.iter()
		.for_each(|(line, token, _)| {
			println!("Line {}: {}", line, token);
		});
}
