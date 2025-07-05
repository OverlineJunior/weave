use logos::Logos;

#[derive(Logos)]
#[logos(skip r"[ \t\n]+")]
enum Token {
	#[token("+")]
	Plus,

	#[token("-")]
	Minus,

	#[token("*")]
	Multiply,

	#[token("/")]
	Divide,

	#[token("(")]
	LParen,

	#[token(")")]
	RParen,

	#[regex("[0-9]+", |lex| lex.slice().parse::<isize>().unwrap())]
	Integer(isize),
}

fn main() {
	let mut lexer = Token::lexer("1 + 2 * (3 - 4)");

	while let Some(token) = lexer.next() {
		println!("{:?}", token);
	}
}