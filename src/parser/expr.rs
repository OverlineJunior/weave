use crate::lexer::value::Value;
use std::fmt::{self, Debug, Display, Formatter};

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
        ty: Ty,
    },
}

impl<Ty> Expr<Ty> {
    pub fn ty(&self) -> &Ty {
        match self {
            Expr::Literal(_, ty) => ty,
            Expr::ComponentCons { ty, .. } => ty,
        }
    }
}

impl<T: Debug> Display for Expr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(val, ty) => {
                write!(f, "Literal(value: {:?}, type: {:?})", val, ty)
            }
            Expr::ComponentCons { name, fields, ty } => {
                write!(f, "ComponentCons(name: {}, fields: {:?}, type: {:?})", name, fields, ty)
            }
        }
    }
}
