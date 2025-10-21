use crate::{interpreter::{ecs::{UserComponentInst, UserWorld}, eval::eval, runtime_error::RuntimeError}, parser::stmt::Stmt};
use flecs_ecs::{core::flecs::Wildcard, prelude::*};

pub fn exec(stmt: &Stmt, ecs: &'static World) -> Result<(), RuntimeError> {
    match stmt {
        Stmt::ComponentDecl { name, field_decls } => {
            ecs.decl_comp_type(name, field_decls.clone())?;
            Ok(())
        }
        Stmt::SystemDecl { name, query, body } => {
            // Maps the query's component type names to actual component types.
            let resolved_query = query
                .iter()
                .map(|(_, comp_name)| {
                    ecs
                        .get_comp_type(comp_name)
                        .map(|comp_type| (comp_name.clone(), comp_type))
                        .ok_or(RuntimeError::UndefinedComponentType {
                            name: comp_name.clone(),
                            line: 555,
                        })
                })
                .collect::<Result<_, _>>()?;

            ecs.decl_system(name, resolved_query)?;

            ecs
                .query::<&(Wildcard, UserComponentInst)>()
                .build()
                .each_iter(|it, idx, comp| {
                    println!("{comp:?}");
                    exec(body, ecs);
                });

            Ok(())
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
