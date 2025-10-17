use flecs_ecs::prelude::*;
use crate::{interpreter::runtime_error::RuntimeError, lexer::value::Value};

#[derive(Component, Debug, Clone, PartialEq)]
pub struct UserVariable {
    pub name: String,
    pub value: Value,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct UserComponent {
    pub type_name: String,
    pub fields: Vec<(String, Value)>,
    pub instance: Component<'static, UserComponent>,
}

pub trait WorldExt {
    fn declare_variable(&'static self, name: &str, value: Value) -> Result<(), RuntimeError>;
    fn get_variable_entity(&self, name: &str) -> Option<EntityView<'static>>;
    fn get_variable_value(&self, name: &str) -> Option<Value>;
}

impl WorldExt for World {
    fn declare_variable(&'static self, name: &str, value: Value) -> Result<(), RuntimeError> {
        if let Some(e) = self.get_variable_entity(name) {
            e.destruct();
        }

        self
            .entity()
            .set(UserVariable {
                name: name.to_string(),
                value,
            });

        Ok(())
    }

    fn get_variable_entity(&self, name: &str) -> Option<EntityView<'static>> {
        self
            .query::<&UserVariable>()
            .build()
            .find(|uv| uv.name == name)
    }

    fn get_variable_value(&self, name: &str) -> Option<Value> {
        let entity = self.get_variable_entity(name)?;
        let mut value = None;

        entity.get::<&UserVariable>(|uv| {
            value = Some(uv.value.clone());
        });

        value
    }
}
