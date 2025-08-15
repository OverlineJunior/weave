use std::collections::HashMap;
use crate::r#type::Type;

#[derive(Debug)]
pub struct TypeEnv {
    types: HashMap<String, Type>,
}

impl TypeEnv {
    pub fn new() -> Self {
        let mut env = Self {
            types: HashMap::new(),
        };
        env.insert("Int".to_string(), Type::Int);
        env.insert("String".to_string(), Type::String);
        env
    }

    pub fn insert(&mut self, name: String, ty: Type) {
        self.types.insert(name, ty);
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }
}
