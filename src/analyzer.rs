use crate::{
    semantic_error::SemanticError,
    stmt::{Field, Stmt},
    r#type::Type,
    type_env::TypeEnv,
};

pub fn analyze(ast: &Stmt<()>, env: &mut TypeEnv) -> Result<Stmt<Type>, SemanticError> {
    match ast {
        Stmt::ComponentDef { name, fields } => {
            let resolved_fields = resolve_fields(fields.clone(), env)?;

            let comp_def = Stmt::ComponentDef {
                name: name.clone(),
                fields: resolved_fields.clone(),
            };

            env.insert(
                name.clone(),
                Type::Component {
                    name: name.clone(),
                    fields: resolved_fields,
                },
            );

            Ok(comp_def)
        }
    }
}

fn resolve_fields(
    fields: Vec<Field<()>>,
    env: &TypeEnv,
) -> Result<Vec<Field<Type>>, SemanticError> {
    fields
        .into_iter()
        .map(|field| {
            let resolved_type =
                env.get(&field.ty_name)
                    .cloned()
                    .ok_or_else(|| SemanticError::UndefinedType {
                        ty_name: field.ty_name.clone(),
                        line: 0,
                    })?;

            Ok(Field {
                name: field.name,
                ty_name: field.ty_name,
                ty: resolved_type,
            })
        })
        .collect()
}
