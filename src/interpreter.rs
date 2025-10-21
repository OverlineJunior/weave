use std::collections::HashMap;
use flecs_ecs::{core::flecs::Wildcard, prelude::*};
use crate::{interpreter::{ecs::UserComponentInst, exec::exec, runtime_error::RuntimeError}, parser::stmt::Stmt};

mod eval;
mod exec;
pub mod ecs;
pub mod runtime_error;

pub fn interpret(ast: &Stmt) -> Result<(), RuntimeError> {
	let mut env = HashMap::new();
	let ecs = Box::leak(Box::new(World::new()));

	exec(ast, &mut env, ecs)?;
	
	println!("Final Environment: {:?}", env);
	println!("Final ECS: {:?}", ecs.to_json_world(None));

	let q = ecs.query::<&(Wildcard, UserComponentInst)>().build();

	q.each_iter(|it, idx, uc| {
		println!("Entity {:?} has UserComponent({}) with fields {:?}", it.entity(idx), uc.type_name, uc.fields);
	});

	Ok(())
}
