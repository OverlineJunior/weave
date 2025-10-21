use flecs_ecs::prelude::*;
use crate::{interpreter::runtime_error::RuntimeError, lexer::value::Value};

#[derive(Component, Debug, Clone, PartialEq)]
pub struct UserVariable {
    pub name: String,
    pub value: Value,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct UserComponentType {
    pub name: String,
    pub field_decls: Vec<String>,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct UserComponentInst {
    pub type_name: String,
    pub fields: Vec<(String, Value)>,
    pub entity: Component<'static, UserComponentInst>,
}

pub trait UserWorld {
    // Variables.
    fn declare_variable(&'static self, name: &str, value: Value);
    fn get_variable_entity(&self, name: &str) -> Option<EntityView<'static>>;
    fn get_variable(&self, name: &str) -> Option<Value>;

    // Component types.
    fn decl_comp_type(&'static self, name: &str, field_decls: Vec<String>) -> Result<(), RuntimeError>;
    fn get_comp_type_entity(&self, name: &str) -> Option<EntityView<'static>>;
    fn get_comp_type(&self, name: &str) -> Option<UserComponentType>;
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

    fn decl_comp_type(&'static self, name: &str, field_decls: Vec<String>) -> Result<(), RuntimeError> {
        if let Some(e) = self.get_comp_type_entity(name) {
            return Err(RuntimeError::ComponentRedeclaration { name: name.to_string(), line: 555 });
        }

        self
            .entity_named(format!("component_type({name})").as_str())
            .set(UserComponentType {
                name: name.to_string(),
                field_decls,
            });

        Ok(())
    }

    fn get_comp_type_entity(&self, name: &str) -> Option<EntityView<'static>> {
        self
            .query::<&UserComponentType>()
            .build()
            .find(|uc| uc.name == name)
    }

    fn get_comp_type(&self, name: &str) -> Option<UserComponentType> {
        let entity = self.get_comp_type_entity(name)?;
        let mut comp = None;

        entity.get::<&UserComponentType>(|uc| {
            comp = Some(uc.clone());
        });

        comp
    }
}

pub trait UserEntity {
    fn set_user_comp(&'_ self, comp_inst: UserComponentInst);
}

impl UserEntity for EntityView<'_> {
    fn set_user_comp(&'_ self, comp_inst: UserComponentInst) {
        let world = self.world();
        let aux_entity = world.entity();
        let comp_entity = comp_inst.entity;
        self.set_id(comp_inst, (aux_entity, comp_entity));
    }
}
