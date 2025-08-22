use crate::{interpreter::exec::exec, parser::stmt::Stmt};

mod eval;
mod exec;

pub fn interpret(ast: &Stmt) -> Result<(), String> {
	exec(ast)?;
	Ok(())
}
