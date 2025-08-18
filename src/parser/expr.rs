use crate::lexer::value::Value;
use std::fmt::{self, Debug, Display, Formatter};

pub type Field = (String, Expr);

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    ComponentCons {
        name: String,
        fields: Vec<Field>,
    },
    EntityCons(Vec<Expr>)
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(value) => {
                write!(f, "Literal(value: {:?})", value)
            }
            Expr::ComponentCons { name, fields } => {
                write!(f, "ComponentCons(name: {}, fields: {:?})", name, fields)
            },
            Expr::EntityCons(exprs) => {
                write!(f, "EntityCons(exprs: {:?})", exprs)
            }
        }
    }
}
