use flecs_ecs::prelude::*;
use crate::lexer::value::Value;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct UserComponent {
    pub fields: Vec<(String, Value)>,
    pub id: Component<'static, UserComponent>,
}
