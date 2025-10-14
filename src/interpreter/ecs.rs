use flecs_ecs::prelude::*;
use crate::lexer::value::Value;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct UserComponent {
    pub type_name: String,
    pub fields: Vec<(String, Value)>,
    pub instance: Component<'static, UserComponent>,
}
