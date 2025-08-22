use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Int(i64),
	String(String),
	Entity(u64),
	Component {
		name: String,
		fields: Vec<(String, Value)>,
	},
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Value::Int(n) => write!(f, "{}", n),
			Value::String(s) => write!(f, "\"{}\"", s),
			Value::Entity(id) => write!(f, "Entity({})", id),
			Value::Component { name, fields } => {
				let fields_str: Vec<String> = fields
					.iter()
					.map(|(field_name, value)| format!("{}: {}", field_name, value))
					.collect();
				write!(f, "{} {{ {} }}", name, fields_str.join(", "))
			}
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
