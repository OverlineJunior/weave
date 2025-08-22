use crate::{interpreter::eval::eval, parser::stmt::Stmt};

pub fn exec(stmt: &Stmt) -> Result<(), String> {
	match stmt {
		Stmt::ComponentDef { name, field_decls } => {
			// TODO! Register component definition in environment.
			println!("Defining component {} with fields: {:?}", name, field_decls);
			Ok(())
		}
		Stmt::SystemDecl { name, query, body } => {
			// TODO! Register system definition in environment.
			println!("Defining system {} with query: {:?}", name, query);
			exec(body)
		}
		Stmt::VarDecl { name, value } => {
			// TODO! Evaluate value and register variable in environment
			println!("Declaring variable {} with value: {:?}", name, value);
			Ok(())
		}
		Stmt::Print(expr) => {
			println!("{:?}", eval(expr)?);
			Ok(())
		}
		Stmt::Block(stmts) => {
			stmts.iter().try_for_each(exec)?;
			Ok(())
		}
		Stmt::Expr(expr) => {
			eval(expr)?;
			Ok(())
		}
	}
}
