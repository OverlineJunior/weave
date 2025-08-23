use crate::{interpreter::eval::eval, lexer::value::Value, parser::stmt::Stmt};
use std::collections::HashMap;

pub fn exec(stmt: &Stmt, env: &mut HashMap<String, Value>) -> Result<(), String> {
    match stmt {
        Stmt::ComponentDecl { name, field_decls } => {
            // TODO! Register component definition in environment.
            println!("Defining component {} with fields: {:?}", name, field_decls);
            Ok(())
        }
        Stmt::SystemDecl { name, query, body } => {
            // TODO! Register system definition in environment.
            println!("Defining system {} with query: {:?}", name, query);
            exec(body, env)
        }
        Stmt::VarDecl { name, value } => {
            println!("Declaring variable {} with value: {:?}", name, value);

			let evaluated_value = eval(value, env)?;
			env.insert(name.clone(), evaluated_value);

            Ok(())
        }
        Stmt::Print(expr) => {
            println!("{:?}", eval(expr, env)?);
            Ok(())
        }
        Stmt::Block(stmts) => {
            stmts.iter().try_for_each(|s| exec(s, env))?;
            Ok(())
        }
        Stmt::Expr(expr) => {
            eval(expr, env)?;
            Ok(())
        }
    }
}
