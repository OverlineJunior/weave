use crate::lexer::{
    lexical_error::LexicalError,
    token::{Spanned, Token},
};
use std::{iter::Peekable, str::Chars};

pub mod lexical_error;
pub mod token;
pub mod value;

pub fn tokenize(source: String) -> Result<Vec<Spanned<Token>>, LexicalError> {
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

    fn next_token(&mut self) -> Result<Option<Spanned<Token>>, LexicalError> {
        if self.chars.peek().is_none() {
            return Ok(None);
        }

        let token = match self.chars.peek().unwrap() {
            // Single lexeme tokens.
            c if is_single_lexeme(*c) => self.next_single_lexeme(),

            // Int.
            c if c.is_ascii_digit() => self.next_int()?,

            // String.
            '"' => self.next_string()?,

            // Keyword, or else, identifier.
            c if is_id_start(*c) => self.next_id(),

            // Newline skip and line increment.
            '\n' => {
                self.chars.next();
                self.line += 1;
                return self.next_token();
            }

            // Whitespace skip.
            c if is_whitespace(*c) => {
                self.chars.next();
                return self.next_token();
            }

            // No match was found.
            c => return Err(LexicalError::UnexpectedChar { ch: *c, line: 1 }),
        };

        Ok(Some(token))
    }

    fn next_single_lexeme(&mut self) -> Spanned<Token> {
        let ch = self.chars.next().unwrap();
        let token = match ch {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ',' => Token::Comma,
            '.' => Token::Dot,
            ':' => Token::Colon,
            '=' => Token::Assign,
            _ => unreachable!(),
        };

        (self.line, token, self.line)
    }

    fn next_int(&mut self) -> Result<Spanned<Token>, LexicalError> {
        assert!(
            self.chars.peek().unwrap().is_ascii_digit(),
            "`next_int` called when next char is not a digit"
        );

        let lexeme = self.eat_while(|c| c.is_ascii_digit());
        let value = lexeme
            .parse::<i64>()
            .unwrap_or_else(|_| panic!("Failed to parse int: {lexeme}"));

        Ok((self.line, value.into(), self.line))
    }

    fn next_string(&mut self) -> Result<Spanned<Token>, LexicalError> {
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
                    return Err(LexicalError::UnterminatedString { line: self.line });
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
        assert!(
            is_id_start(*self.chars.peek().unwrap()),
            "`next_id` called when next char is not a valid identifier start"
        );

        let lexeme = self.eat_while(|c| is_id_start(c) || is_id_continue(c));
        let token = Token::keyword_from(&lexeme).unwrap_or(Token::Id(lexeme.to_string()));

        (self.line, token, self.line)
    }

    fn eat_while<F>(&mut self, predicate: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut lexeme = String::new();

        while let Some(&ch) = self.chars.peek() {
            if predicate(ch) {
                lexeme.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }

        lexeme
    }
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

fn is_single_lexeme(c: char) -> bool {
    matches!(c, '(' | ')' | '{' | '}' | ',' | '.' | ':' | '=')
}
