use crate::lexer::tokenize;

mod value;
mod token;
mod lexing_error;
mod lexer;

const SOURCE: &str = r#"
	55 entity "foo bar" var1 123
	"This is a test string" system
	_var2
"#;

fn main() {
	tokenize(SOURCE.to_string())
		.expect("Failed to tokenize source")
		.iter()
		.for_each(|(line, token, _)| {
			println!("Line {}: {}", line, token);
		});
}
