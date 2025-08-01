use crate::literal::Literal;
use logos::Logos;

#[derive(Logos, Debug)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex("[0-9]+", |lex| Literal::Number(lex.slice().parse::<f64>().unwrap()))]
    Number(Literal),
    #[regex(r#""[^"]*""#, |lex| Literal::String(lex.slice().trim_matches('"').into()))]
    String(Literal),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Id(String),

	#[token("system")]
	System,

	#[token("(")]
	LPar,
	#[token(")")]
	RPar,
	#[token("{")]
	LBrace,
	#[token("}")]
	RBrace,
}
