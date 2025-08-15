use crate::{
    stmt::{Field, Stmt},
    r#type::Type, type_env::TypeEnv,
};

pub fn analyze(ast: &Stmt<()>, env: &mut TypeEnv) -> Result<Stmt<Type>, String> {
    match ast {
        Stmt::ComponentDef { name, fields } => {
			let resolved_fields = resolve_fields(fields.clone(), &env)?;

			let comp_def = Stmt::ComponentDef {
				name: name.clone(),
				fields: resolved_fields.clone(),
			};

			env.insert(name.clone(), Type::Component {
				name: name.clone(),
				fields: resolved_fields,
			});

			Ok(comp_def)
		},
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
