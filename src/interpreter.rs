use flecs_ecs::prelude::*;
use crate::{interpreter::{exec::exec, runtime_error::RuntimeError}, parser::stmt::Stmt};

mod eval;
mod exec;
pub mod ecs;
pub mod runtime_error;

pub fn interpret(ast: &Stmt) -> Result<(), RuntimeError> {
	let ecs = Box::leak(Box::new(World::new()));

	exec(ast, ecs)?;

	println!("------------------------------------------------------------");
	println!("Final ECS: {:?}", ecs.to_json_world(None));

	Ok(())
}
