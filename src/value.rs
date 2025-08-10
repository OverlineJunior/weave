use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Value {
	Int(i64),
	String(String),
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Value::Int(n) => write!(f, "{}", n),
			Value::String(s) => write!(f, "\"{}\"", s),
		}
	}
}

impl From<i64> for Value {
	fn from(value: i64) -> Self {
		Value::Int(value)
	}
}

impl From<String> for Value {
	fn from(value: String) -> Self {
		Value::String(value)
	}
}
