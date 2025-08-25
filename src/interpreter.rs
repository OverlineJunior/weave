use std::collections::HashMap;

use crate::{interpreter::exec::exec, parser::stmt::Stmt};

mod eval;
mod exec;

pub fn interpret(ast: &Stmt) -> Result<(), String> {
	let mut env = HashMap::new();
	exec(ast, &mut env)?;
	Ok(())
}
