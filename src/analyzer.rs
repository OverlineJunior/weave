use std::collections::HashMap;

use crate::{
    stmt::{Field, Stmt},
    r#type::Type,
};

struct TypeEnv {
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

pub fn analyze(ast: &Stmt<()>) -> Result<Stmt<Type>, String> {
    let env = TypeEnv::new();

    match ast {
        Stmt::ComponentDef { name, fields } => Ok(Stmt::ComponentDef {
            name: name.clone(),
            fields: resolve_fields(fields.clone(), &env)?,
        }),
    }
}

fn resolve_fields(fields: Vec<Field<()>>, env: &TypeEnv) -> Result<Vec<Field<Type>>, String> {
    fields
        .into_iter()
        .map(|field| {
            let resolved_type = env
                .get(&field.ty_name)
                .cloned()
                .ok_or_else(|| format!("Unknown type: {}", field.ty_name))?;

            Ok(Field {
                name: field.name,
                ty_name: field.ty_name,
                ty: resolved_type,
            })
        })
        .collect()
}
