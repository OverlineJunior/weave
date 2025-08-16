use crate::{
    lexer::value::Value, parser::{expr::{DataField, Expr}, stmt::{Stmt, TypeField}}, semantic_error::SemanticError, r#type::Type, type_env::TypeEnv
};

pub fn analyze(ast: &Stmt, env: &mut TypeEnv) -> Result<Stmt<Type>, SemanticError> {
    match ast {
        Stmt::Block(stmts) => {
            let analyzed_stmts = stmts
                .iter()
                .map(|stmt| analyze(stmt, env))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Stmt::Block(analyzed_stmts))
        }
        Stmt::ComponentDef { name, fields } => {
            let resolved_fields = resolve_type_fields(fields.clone(), env)?;

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
        Stmt::Expr(expr) => {
            let analyzed_expr = analyze_expr(expr, env)?;
            Ok(Stmt::Expr(analyzed_expr))
        }
    }
}

fn analyze_expr(expr: &Expr<()>, env: &TypeEnv) -> Result<Expr<Type>, SemanticError> {
    match expr {
        Expr::Literal(val, _) => {
            let ty = match &val {
                Value::Int(_) => Type::Int,
                Value::String(_) => Type::String,
            };
            Ok(Expr::Literal(val.clone(), ty))
        }
        Expr::ComponentCons { name, fields } => {
            let resolved_fields = resolve_data_fields(fields.clone(), env)?;

            let comp_cons = Expr::ComponentCons {
                name: name.clone(),
                fields: resolved_fields,
            };

            // TODO! Do env stuff. Gotta edit resolve_data_fields.

            Ok(comp_cons)
        }
    }
}

fn resolve_type_fields(
    fields: Vec<TypeField>,
    env: &TypeEnv,
) -> Result<Vec<TypeField<Type>>, SemanticError> {
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

            Ok(TypeField {
                name: field.name,
                ty_name: field.ty_name,
                ty: resolved_type,
            })
        })
        .collect()
}

fn resolve_data_fields(
    fields: Vec<DataField>,
    env: &TypeEnv,
) -> Result<Vec<DataField<Type>>, SemanticError> {
    fields
        .into_iter()
        .map(|field| {
            let analyzed_data = analyze_expr(&field.data, env)?;
            Ok(DataField {
                name: field.name,
                data: analyzed_data,
            })
        })
        .collect()
}
