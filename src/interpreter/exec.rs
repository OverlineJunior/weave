use crate::{interpreter::{eval::eval, runtime_error::RuntimeError}, lexer::value::Value, parser::stmt::Stmt};
use std::collections::HashMap;
use flecs_ecs::prelude::*;

pub fn exec(
    stmt: &Stmt,
    env: &mut HashMap<String, Value>,
    ecs: &'static World,
) -> Result<(), RuntimeError> {
    match stmt {
        Stmt::ComponentDecl { name, field_decls } => {
            env.insert(
                name.clone(),
                Value::ComponentType {
                    name: name.clone(),
                    field_decls: field_decls.clone()
                }
            );

            Ok(())
        }
        Stmt::SystemDecl { name, query, body } => {
            // TODO! Register system definition in environment.
            println!("Defining system {} with query: {:?}", name, query);
            exec(body, env, ecs)
        }
        Stmt::VarDecl { name, value } => {
			let evaluated_value = eval(value, env, ecs)?;
			env.insert(name.clone(), evaluated_value);

            Ok(())
        }
        Stmt::Print(exprs) => {
            for expr in exprs {
                println!("{}", eval(expr, env, ecs)?);
            }
            Ok(())
        }
        Stmt::Block(stmts) => {
            stmts.iter().try_for_each(|s| exec(s, env, ecs))?;
            Ok(())
        }
        Stmt::Expr(expr) => {
            eval(expr, env, ecs)?;
            Ok(())
        }
    }
}
