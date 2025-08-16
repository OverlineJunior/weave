use std::fmt::{self, Debug, Display, Formatter};

use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct DataField<Ty = ()> {
	pub name: String,
	pub data: Expr<Ty>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<Ty> {
	Literal(Value, Ty),
	ComponentCons {
		name: String,
		fields: Vec<DataField<Ty>>,
	},
}

impl<T: Debug> Display for Expr<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Expr::Literal(val, ty) => {
				write!(f, "Literal(value: {:?}, type: {:?})", val, ty)
			},
			Expr::ComponentCons { name, fields } => {
				write!(f, "ComponentCons(name: {}, fields: {:?})", name, fields)
			},
		}
	}
}
