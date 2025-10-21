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
            ecs.decl_comp_type(name, field_decls.clone())?;
            Ok(())
        }
        Stmt::SystemDecl { name, query, body } => {
            // TODO! Register system definition in environment.
            println!("Defining system {} with query: {:?}", name, query);
            exec(body, env, ecs)
        }
        Stmt::VarDecl { name, value } => {
			ecs.decl_var(name, eval(value, env, ecs)?);
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
