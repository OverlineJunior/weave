use std::collections::HashMap;
use flecs_ecs::prelude::*;
use crate::{interpreter::{exec::exec, runtime_error::RuntimeError}, parser::stmt::Stmt};

mod eval;
mod exec;
pub mod ecs;
pub mod runtime_error;

pub fn interpret(ast: &Stmt) -> Result<(), RuntimeError> {
	let mut env = HashMap::new();
	let ecs = Box::leak(Box::new(World::new()));

	exec(ast, &mut env, ecs)?;

	println!("------------------------------------------------------------");
	println!("Final Environment: {:?}", env);
	println!("Final ECS: {:?}", ecs.to_json_world(None));

	Ok(())
}
