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

pub trait UserWorld {
    // Variables.
    fn declare_variable(&'static self, name: &str, value: Value);
    fn get_variable_entity(&self, name: &str) -> Option<EntityView<'static>>;
    fn get_variable(&self, name: &str) -> Option<Value>;

    // Components.
    fn declare_component(&'static self, name: &str, fields: Vec<(String, Value)>) -> Result<UserComponent, RuntimeError>;
    fn get_component_entity(&self, name: &str) -> Option<EntityView<'static>>;
    fn get_component(&self, name: &str) -> Option<UserComponent>;
}

impl UserWorld for World {
    fn declare_variable(&'static self, name: &str, value: Value) {
        if let Some(e) = self.get_variable_entity(name) {
            e.destruct();
        }

        self
            .entity()
            .set(UserVariable {
                name: name.to_string(),
                value,
            });
    }

    fn get_variable_entity(&self, name: &str) -> Option<EntityView<'static>> {
        self
            .query::<&UserVariable>()
            .build()
            .find(|uv| uv.name == name)
    }

    fn get_variable(&self, name: &str) -> Option<Value> {
        let entity = self.get_variable_entity(name)?;
        let mut value = None;

        entity.get::<&UserVariable>(|uv| {
            value = Some(uv.value.clone());
        });

        value
    }

    fn declare_component(&'static self, name: &str, fields: Vec<(String, Value)>) -> Result<UserComponent, RuntimeError> {
        Ok(UserComponent {
            type_name: name.to_string(),
            fields: fields,
            instance: self.component_named::<UserComponent>(format!("user_component({name})").as_str()),
        })
    }

    fn get_component_entity(&self, name: &str) -> Option<EntityView<'static>> {
        self
            .query::<&UserComponent>()
            .build()
            .find(|uc| uc.type_name == name)
    }

    fn get_component(&self, name: &str) -> Option<UserComponent> {
        let entity = self.get_component_entity(name)?;
        let mut comp = None;

        entity.get::<&UserComponent>(|uc| {
            comp = Some(uc.clone());
        });

        comp
    }
}
