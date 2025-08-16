use crate::lexer::value::Value;
use std::fmt::{self, Display, Formatter};

pub type Spanned<T> = (usize, T, usize);

fn uppercase_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int(Value),
    String(Value),
    Id(String),

    Entity,
    Component,
    System,

    LPar,   // (
    RPar,   // )
    LBrace, // {
    RBrace, // }
    Comma,  // ,
    Dot,    // .
    Colon,  // :
}

impl Token {
    // Cannot be exhaustive, so never forget to add new keywords here.
    pub fn keyword_from(s: &str) -> Option<Token> {
        match s {
            "entity" => Some(Token::Entity),
            "component" => Some(Token::Component),
            "system" => Some(Token::System),
            _ => None,
        }
    }

    pub fn lexeme(&self) -> String {
        match self {
            Token::Int(val) => val.to_string(),
            Token::String(val) => val.to_string(),
            Token::Id(name) => name.clone(),
            Token::Entity => "entity".to_string(),
            Token::Component => "component".to_string(),
            Token::System => "system".to_string(),
            Token::LPar => "(".to_string(),
            Token::RPar => ")".to_string(),
            Token::LBrace => "{".to_string(),
            Token::RBrace => "}".to_string(),
            Token::Comma => ",".to_string(),
            Token::Dot => ".".to_string(),
            Token::Colon => ":".to_string(),
        }
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Token::Int { .. } | Token::String { .. })
    }

    pub fn is_statement(&self) -> bool {
        matches!(self, Token::Entity | Token::Component | Token::System)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Token::Int(val) => write!(f, "Int({})", val),
            Token::String(val) => write!(f, "String({})", val),
            Token::Id(name) => write!(f, "Id({})", name),
            _ => uppercase_first(&self.lexeme()).fmt(f),
        }
    }
}

impl From<i64> for Token {
    fn from(value: i64) -> Self {
        Token::Int(Value::Int(value))
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Token::String(Value::String(value))
    }
}
