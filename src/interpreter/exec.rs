use crate::{interpreter::{ecs::UserWorld, eval::eval, runtime_error::RuntimeError}, lexer::value::Value, parser::stmt::Stmt};
use std::collections::HashMap;
use flecs_ecs::prelude::*;

pub fn exec(
    stmt: &Stmt,
    env: &mut HashMap<String, Value>,
    ecs: &'static World,
) -> Result<(), RuntimeError> {
    match stmt {
        Stmt::ComponentDecl { name, field_decls } => {
            // TODO! Use ecs to declare component instead. Just note that the component structs and related functions are wrong and need rethinking, specially when it comes to component type vs instance.
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
			ecs.declare_variable(name, eval(value, env, ecs)?);
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
