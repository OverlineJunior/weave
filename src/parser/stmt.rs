use std::fmt::{self, Debug, Display, Formatter};

use crate::parser::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct TypeField<Ty = ()> {
	pub name: String,
	pub ty_name: String,
	pub ty: Ty,
}

#[derive(Debug, Clone)]
pub enum Stmt<Ty = ()> {
	Block(Vec<Stmt<Ty>>),
	Expr(Expr<Ty>),
	ComponentDef {
		name: String,
		fields: Vec<TypeField<Ty>>,
	},
}

impl<T: Debug> Display for Stmt<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Stmt::Block(stmts) => {
				write!(f, "Block(stmts: {:?})", stmts)
			},
			Stmt::Expr(expr) => {
				write!(f, "Expr(expr: {})", expr)
			},
			Stmt::ComponentDef { name, fields } => {
				write!(f, "ComponentDef(name: {}, fields: {:?})", name, fields)
			},
		}
	}
}
