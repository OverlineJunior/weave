use std::fmt::{self, Debug, Display, Formatter};
use crate::r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Field<Ty> {
	pub name: String,
	pub ty_name: String,
	pub ty: Ty,
}

#[derive(Debug, Clone)]
pub enum Stmt<Ty> {
	ComponentDef {
		name: String,
		fields: Vec<Field<Ty>>,
	}
}

impl<T: Debug> Display for Stmt<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Stmt::ComponentDef { name, fields } => {
				write!(f, "ComponentDef(name: {}, fields: {:?})", name, fields)
			}
		}
	}
}
