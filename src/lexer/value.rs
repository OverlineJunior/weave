use std::fmt::{self, Display, Formatter};

use flecs_ecs::core::EntityView;

use crate::interpreter::ecs::UserComponent;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    String(String),
    Entity(EntityView<'static>),
	ComponentType { name: String },
    ComponentInst {
        type_name: String,
        fields: Vec<(String, Value)>,
        component: UserComponent,
    },
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Entity(id) => write!(f, "Entity({})", id),
			Value::ComponentType { name } => write!(f, "ComponentType({})", name),
            Value::ComponentInst { type_name, fields, component } => {
                let fields_str: Vec<String> = fields
                    .iter()
                    .map(|(field_name, value)| format!("{}: {}", field_name, value))
                    .collect();
                write!(f, "ComponentInst({}, {}, {:?})", type_name, fields_str.join(", "), component)
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
