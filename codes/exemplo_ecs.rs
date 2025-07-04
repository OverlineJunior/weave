// Componentes podem ser representados através de simples structs.
struct Position { x: f32, y: f32 }

struct Velocity { dx: f32, dy: f32 }

// Sistemas podem ser representados como funções que processam todas as
// entidades e seus respectivos componentes.
fn apply_velocity(
	entities: &[usize],
	positions: &mut [Position],
	velocities: &[Velocity]
) {
	for &entity in entities {
		positions[entity].x += velocities[entity].dx;
		positions[entity].y += velocities[entity].dy;
	}
}

fn main() {
	// Entidades podem ser representadas como números únicos.
	let entities = [0, 1];

	// Cada componente é armazenado em um vetor separado, onde o índice
	// representa a entidade e o valor representa seu componente.
	let mut positions = [
		Position { x: 0.0, y: 0.0 }, // Entidade 0.
		Position { x: 1.0, y: 1.0 }, // Entidade 1.
	];

	let velocities = [
		Velocity { dx: 1.0, dy: 1.0 }, // Entidade 0.
		Velocity { dx: 2.0, dy: 2.0 }, // Entidade 1.
	];

	// No padrão ECS, é muito comum que os sistemas sejam executados repetidamente.
	loop {
		apply_velocity(&entities, &mut positions, &velocities);
	}
}