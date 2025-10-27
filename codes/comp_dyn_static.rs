// Definição de um componente estático.
#[derive(Component)]
struct Static {
	value: i32,
}

fn main() {
	let mut world = World::new();

	// Inserção de um componente estático.
	world
		.entity()
		.set(Static { value: 10 });

	// Definição de um componente dinâmico.
	let dynamic = world
		.component_untyped()
		.member(i32::id(), "value");

	unsafe {
		// Inserção de um componente dinâmico.
		world
			.entity()
			.add_id_unchecked(dynamic);
	}
}
