use std::{iter::Peekable, str::Chars};
use crate::{lexing_error::LexingError, token::{Spanned, Token}, value::Value};

pub fn tokenize(source: String) -> Result<Vec<Spanned<Token>>, LexingError> {
	let mut lexer = Lexer::new(&source);
	let mut tokens = Vec::new();

	loop {
		match lexer.next_token() {
			Ok(Some(token)) => tokens.push(token),
			Ok(None) => break,
			Err(e) => return Err(e),
		}
	}

	Ok(tokens)
}

struct Lexer<'a> {
	chars: Peekable<Chars<'a>>,
	line: usize,
}

impl<'a> Lexer<'a> {
	fn new(source: &'a str) -> Self {
		Self {
			chars: source.chars().peekable(),
			line: 1,
		}
	}

	fn next_token(&mut self) -> Result<Option<Spanned<Token>>, LexingError> {
        if self.chars.peek().is_none() {
            return Ok(None);
        }

        let token = match self.chars.peek().unwrap() {
            c if c.is_ascii_digit() => self.next_number()?,
            '"' => self.next_string()?,

            //c if is_id_start(*c) => self.next_id(),

            '\n' => {
                self.chars.next();
                self.line += 1;
                return self.next_token();
            }

            c if is_whitespace(*c) => {
                self.chars.next();
                return self.next_token();
            }

            c => return Err(LexingError::UnexpectedChar { ch: *c, line: 1 }),
        };

        Ok(Some(token))
    }

	fn next_number(&mut self) -> Result<Spanned<Token>, LexingError> {
		assert!(
			self.chars.peek().unwrap().is_ascii_digit(),
			"`next_number` called when next char is not a digit"
		);

		let lexeme = eat_while(&mut self.chars, |c| c.is_ascii_digit());
		let value = lexeme
			.parse::<i64>()
			.unwrap_or_else(|_| panic!("Failed to parse int: {lexeme}"));

		Ok((self.line, value.into(), self.line))
	}

	fn next_string(&mut self) -> Result<Spanned<Token>, LexingError> {
		assert_eq!(
			self.chars.peek().unwrap(),
			&'"',
			"`next_string` called when next char is not `\"`"
		);

		self.chars.next(); // Consume the opening quote.

		let mut lexeme = String::new();
		while let Some(&ch) = self.chars.peek() {
			match ch {
				'"' => {
					self.chars.next(); // Consume the closing quote.
					break;
				}
				'\n' => {
					return Err(LexingError::UnterminatedString { line: self.line });
				}
				_ => {
					lexeme.push(ch);
					self.chars.next();
				}
			}
		}

		Ok((self.line, lexeme.into(), self.line))
	}

	fn next_id(&mut self) -> Spanned<Token> {
		todo!();
	}
}

fn eat_while<F>(chars: &mut Peekable<Chars>, predicate: F) -> String
where
	F: Fn(char) -> bool,
{
	let mut lexeme = String::new();

	while let Some(&ch) = chars.peek() {
		if predicate(ch) {
			lexeme.push(ch);
			chars.next();
		} else {
			break;
		}
	}

	lexeme
}

fn is_id_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_id_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\r' | '\t')
}
