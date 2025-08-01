mod literal;
mod token;

use token::Token;
use logos::Logos;

const SOURCE: &str = r#"
    system add_inversion() {

    }
"#;

fn main() {
    let token_iter = Token::lexer(SOURCE);

    token_iter.for_each(|t| {
        println!("{:?}", t.unwrap());
    });
}
