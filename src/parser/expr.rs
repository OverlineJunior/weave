use crate::lexer::value::Value;
use std::fmt::{self, Debug, Display, Formatter};

pub type Field = (String, Expr);

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    ComponentCons {
        type_name: String,
        fields: Vec<Field>,
    },
    EntityCons(Vec<Expr>),
    Var { name: String },
    ComponentFieldGet {
        type_name: String,
        field_name: String,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(value) => {
                write!(f, "Literal(value: {:?})", value)
            }
            Expr::ComponentCons { type_name, fields } => {
                write!(f, "ComponentCons(type_name: {}, fields: {:?})", type_name, fields)
            },
            Expr::EntityCons(exprs) => {
                write!(f, "EntityCons(exprs: {:?})", exprs)
            },
            Expr::Var { name } => {
                write!(f, "Var(name: {})", name)
            },
            Expr::ComponentFieldGet { type_name, field_name } => {
                write!(f, "ComponentFieldGet(type_name: {}, field_name: {})", type_name, field_name)
            },
        }
    }
}
