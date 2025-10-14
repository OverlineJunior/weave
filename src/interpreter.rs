use std::collections::HashMap;
use flecs_ecs::{core::flecs::Wildcard, prelude::*};
use crate::{interpreter::{ecs::UserComponent, exec::exec}, parser::stmt::Stmt};

mod eval;
mod exec;
pub mod ecs;

pub fn interpret(ast: &Stmt) -> Result<(), String> {
	let mut env = HashMap::new();
	let ecs = Box::leak(Box::new(World::new()));

	exec(ast, &mut env, ecs)?;
	
	println!("Final Environment: {:?}", env);
	println!("Final ECS: {:?}", ecs.to_json_world(None));

	let q = ecs.query::<&(Wildcard, UserComponent)>().build();

	q.each_iter(|it, idx, uc| {
		println!("Entity {:?} has UserComponent({}) with fields {:?}", it.entity(idx), uc.type_name, uc.fields);
	});

	Ok(())
}
