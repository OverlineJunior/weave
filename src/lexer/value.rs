use std::fmt::{self, Display, Formatter};

use flecs_ecs::core::EntityView;

use crate::interpreter::ecs::UserComponentInst;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    String(String),
    Entity(EntityView<'static>),
    Component(UserComponentInst),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Entity(id) => write!(f, "Entity({})", id),
            Value::Component(comp) => {
                let fields_str: Vec<String> = comp.fields
                    .iter()
                    .map(|(field_name, value)| format!("{}: {}", field_name, value))
                    .collect();
                write!(f, "Component({}, {}, {})", comp.type_name, fields_str.join(", "), comp.entity)
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
