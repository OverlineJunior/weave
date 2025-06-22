#import "../style.typ": figura_legendada

= Fundamentos

== Padrão de Design de Software

=== Definição

- Solução reutilizável para um problema recorrente no design de software;
- É uma descrição geral de como resolver o problema, e não uma solução específica;
- Dividido em padrões _criacionais_, _estruturais_ e _comportamentais_;
- Exemplos: _Singleton_, _Decorator_ e _Observer_.

== Padrão de Arquitetura de Software

=== Definição

- Muito parecido com um padrão de design, só que mais abrangente, vendo a aplicação como um todo;
- Exemplos: _Model-View-Controller_, _Microservices_ e _Entity Component System_.

== Paradigma de Programação

=== Definição

- Conjunto de conceitos e princípios que orientam o desenvolvimento de software;
- Pode ser visto como uma hierarquia, onde há paradigmas mais abrangentes e outros mais específicos, que herdam dos mais abrangentes;
- Os dois paradigmas mais abrangentes são o _imperativo_ e _declarativo_;
- Paradigmas mais específicos incluem o _funcional_, _orientado a objetos_, entre outros.

=== Hierarquia

#figura_legendada(
	image("../../images/hierarquia_paradigmas.png", height: 80%),
	[Fonte: adaptado de #cite(<whatsprogrammingparadigm>, form: "prose").]
)

== _Entity Component System_

=== Definição

- Padrão de arquitetura baseado no design orientado a dados;

- Surgiu na área de desenvolvimento de jogos devido a sua flexibilidade e desempenho;

- Composto de três elementos fundamentais: _entidades_, _componentes_ e _sistemas_;

- Ainda está em fase de formalização.

#pagebreak()

=== Os Três Elementos Fundamentais

- *Entidade:* identificador único sem dado ou lógica;

- *Componente:* estrutura de dados que representa um conceito;

- *Sistema:* lógica que opera em um conjunto de entidades com componentes específicos.

#pagebreak()

#figura_legendada(
	image("../../images/diagrama_ecs.png", height: 75%),
	[Fonte: adaptado de #cite(<ecsfaq>, form: "prose").]
)

#pagebreak()

```rust
struct Position {
	x: f32,
	y: f32,
}

struct Velocity {
	dx: f32,
	dy: f32,
}
```

#pagebreak()

```rust
fn main() {
		let entities = [0, 1];

		let mut positions = [
				Position { x: 0.0, y: 0.0 }, // Entidade 0.
				Position { x: 1.0, y: 1.0 }, // Entidade 1.
		];

		let velocities = [
				Velocity { dx: 1.0, dy: 1.0 }, // Entidade 0.
				Velocity { dx: 2.0, dy: 2.0 }, // Entidade 1.
		];

		loop { apply_velocity(&entities, &mut positions, &velocities); }
}
```

#pagebreak()

```rust
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
```

#pagebreak()

#grid(
	columns: (1fr, 1fr),
	[
		=== Agendador

		Construto com a finalidade de executar os sistemas da aplicação, também determinando a ordem e a frequência de execução.
	],
	image("../../images/diagrama_agendador.png"),
)

#pagebreak()

#grid(
	columns: (1fr, 1fr),
	[
		=== Depurador de ECS

		Interface, gráfica ou não, que permite visualizar e manipular as entidades, componentes e sistemas da aplicação.
	],
	figura_legendada(
		image("../../images/flecs_explorer.png"),
		[Fonte: #cite(<flecsexplorer>, form: "prose").]
	),
)

#pagebreak()

=== Relacionamentos

Conceito que permite o relacionamento entre entidades e componentes, tornando possível a representação de estruturas mais complexas, como hierarquias.

Um caso de uso é a representação de um sistema de arquivos, onde pastas podem conter arquivos — uma relação pai-filho.

#pagebreak()

=== Relacionamentos

#figura_legendada(
	image("../../images/entidade_posicao.png"),
	[```ts Entidade.set<Posição>({0, 0});```],
	[Fonte: elaborado com base em #cite(<entityrelationships>, form: "prose").],
)

#pagebreak()

=== Relacionamentos

#figura_legendada(
	image("../../images/relacionamento_entidades.png"),
	[```cs Terra.add(FilhoDe, Sol); Lua.add(FilhoDe, Terra);```],
	[Fonte: elaborado com base em #cite(<entityrelationships>, form: "prose").],
)

== Interpretador _Tree-Walking_

=== Definição

- Interpretador é um programa que executa diretamente o código fonte de uma linguagem de programação, linha por linha.

- _Tree-Walking_ é uma variante de interpretador com foco em simplicidade (a custo de desempenho).

- É separado em três fases: _análise léxica_, _análise sintática_ e _interpretação_.

#pagebreak()

=== Visão Geral

#figura_legendada(
	image("../../images/mapa_interpretador.png", height: 65%),
	[Fonte: #cite(<craftinginterpreters>, form: "prose").],
)

=== Análise Léxica

#figura_legendada(
	image("../../images/analise_lexica.png", height: 80%),
	[Fonte: baseado em #cite(<craftinginterpreters>, form: "prose").],
)

=== Análise Sintática

#figura_legendada(
	image("../../images/analise_sintatica.png", height: 80%),
	[Fonte: baseado em #cite(<craftinginterpreters>, form: "prose").],
)

=== Interpretação

#figura_legendada(
	image("../../images/interpretacao.png", height: 80%),
	[Fonte: baseado em #cite(<craftinginterpreters>, form: "prose").],
)

= Tecnologias

== Rust

=== Motivação

- *Tipagem forte:* gera garantias em tempo de compilação. Uso de enum e match é ótimo na manipulação dos tokens;

- *Tratamento de erros explícito:* toda etapa de um interpretador está sujeita a erros — tratamento explícito garante que os erros sejam tratados de acordo;

- *Alto desempenho:* desempenho anda lado a lado com ECS e interpretadores.

== Cargo

- Gerenciador de pacotes da linguagem Rust;

- Será utilizado para gerenciar as bibliotecas _Logos_ e _Chumsky_.

== Logos

=== Definição

Biblioteca de análise léxica baseada em _regex_ para Rust.

=== Motivação

A escolha se deve ao fato de que _Logos_ é atualmente uma das bibliotecas mais maturas e completas para análise léxica na linguagem Rust, além da alta compatibilidade com _Chumsky_.

Como bônus, ela é também a biblioteca mais rápida segundo o _benchmark_ disponível no repositório oficial.

=== _Lexer_ para uma Linguagem de Aritmética com _Logos_

#figura_legendada(
	```rust
	#[derive(Logos)]
	#[logos(skip r"[\s]+")]
	enum Token {
		#[token("-")]
		Sub,

		#[regex("[0-9]+", |lex| lex.slice().parse().unwrap())]
		Int(i64),
	}
	```,
	[Fonte: adaptado de #cite(<logos>, form: "prose").],
)

== Chumsky

=== Definição

Biblioteca de análise sintática baseada no paradigma declarativo para Rust.

=== Motivação

A escolha se deve aos mesmos motivos que levaram à escolha de _Logos_: maturidade, compatibilidade e desempenho.

#v(2em)

=== _AST_ e _Parser_ para uma Linguagem de Aritmética com _Chumsky_

```rust
enum Expr<'a> {
	Int(i64),
	Neg(Box<Expr<'a>>),
}
```

#figura_legendada(
	```rust
	fn parser<'a>() -> impl Parser<'a, &'a str, Expr<'a>> {
		let op = |c| just(c).padded()

		// int -> regex([0-9]+)
		let int = text::int(10)
			.map(|s: &str| Expr::Int(s.parse().unwrap()));

		// unary -> int | '-' unary
		let unary = op('-')
			.repeated()
			.foldr(int, |_op, rhs| Expr::Neg(Box::new(rhs)));

		unary
	}
	```,
	[Fonte: adaptado de #cite(<chumsky>, form: "prose").],
)
