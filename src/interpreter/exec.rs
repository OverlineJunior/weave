use crate::{interpreter::{ecs::UserWorld, eval::eval, runtime_error::RuntimeError}, parser::stmt::Stmt};
use flecs_ecs::prelude::*;

pub fn exec(stmt: &Stmt, ecs: &'static World) -> Result<(), RuntimeError> {
    match stmt {
        Stmt::ComponentDecl { name, field_decls } => {
            ecs.decl_comp_type(name, field_decls.clone())?;
            Ok(())
        }
        Stmt::SystemDecl { name, query, body } => {
            // TODO! Register system definition in ecs.
            println!("Defining system {} with query: {:?}", name, query);
            exec(body, ecs)
        }
        Stmt::VarDecl { name, value } => {
			ecs.decl_var(name, eval(value, ecs)?);
            Ok(())
        }
        Stmt::Print(exprs) => {
            for expr in exprs {
                println!("{}", eval(expr, ecs)?);
            }
            Ok(())
        }
        Stmt::Block(stmts) => {
            stmts.iter().try_for_each(|s| exec(s, ecs))?;
            Ok(())
        }
        Stmt::Expr(expr) => {
            eval(expr, ecs)?;
            Ok(())
        }
    }
}
