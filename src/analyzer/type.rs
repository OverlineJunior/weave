use crate::parser::stmt::TypeField;
use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    String,
    Component {
        name: String,
        fields: Vec<TypeField<Type>>,
    },
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::String => write!(f, "String"),
            Type::Component { name, fields } => {
                write!(f, "Component(name: {}, fields: {:?})", name, fields)
            }
        }
    }
}
