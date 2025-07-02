#import "../style.typ": figura_legendada, figura_legendada_titulada
#import "@preview/touying:0.6.1": *

// ≈ 7-9 minutos

= Fundamentos

== Padrão de Design de Software

#grid(
	columns: (1fr, 1fr),
	rows: 1,
	column-gutter: 30pt,
	[Solução reutilizável para um problema recorrente no design de software.],
	figura_legendada_titulada(
		[Exemplo: _Observer_],
		image("../../images/observer.png", height: 75%),
		[Fonte: #cite(form: "prose", <refactoringguru>)],
	),
)

== Padrão de Arquitetura de Software

#grid(
	columns: (1fr, 1fr),
	rows: 1,
	column-gutter: 30pt,
	[Parecido com um padrão de design, mas focado na organização de toda a aplicação.],
	figura_legendada_titulada(
		[Exemplo: _Model-View-Controller_],
		image("../../images/mvc.png", height: 66%),
		[Fonte: baseado em #cite(form: "full", <wikipediamvc>)],
	),
)

== Paradigma de Programação

Conjunto de conceitos e princípios que orientam o desenvolvimento de software.

#figura_legendada(
	image("../../images/hierarquia_paradigmas.png", height: 55%),
	[Fonte: adaptado de #cite(<whatsprogrammingparadigm>, form: "prose").]
)

== _Entity Component System_

- Padrão de arquitetura com origem na indústria de jogos.

- Visa separar dados e lógica, promovendo *flexibilidade* e *adaptação*.

- Separa a aplicação em três elementos fundamentais: *entidades*, *componentes* e *sistemas*.

#pagebreak()

#grid(
	columns: (1.13fr, 1fr),
	rows: 1,
	column-gutter: 30pt,
	[
		=== Os Três Elementos Fundamentais

		- *Entidade:* identificador único sem dado ou lógica;

		- *Componente:* estrutura de dados que representa um conceito;

		- *Sistema:* lógica que opera em um conjunto de entidades com componentes específicos.
	],
	figura_legendada(
		image("../../images/diagrama_ecs.png", height: 76%),
		[Fonte: adaptado de #cite(<ecsfaq>, form: "prose").]
	)
)

#pagebreak()

#align(center)[#image("../../images/mario.jpg", height: 84%)]

#pagebreak()

=== Tudo é uma Entidade

- Componentes e sistemas são associados a entidades.

- Permite a atribuição de metadados aos componentes e sistemas da aplicação.

- Torna a linguagem homoicônica.

#pagebreak()

=== Relacionamentos

Permite o relacionamento entre entidades e componentes, tornando possível a representação de estruturas mais complexas, como hierarquias.

#figura_legendada(
	image("../../images/relacionamento_entidades.png", height: 37%),
	[Fonte: elaborado com base em #cite(<entityrelationships>, form: "prose").],
)

== Interpretador _Tree-Walking_

- Programa que executa diretamente o código fonte de uma linguagem de programação, linha por linha.

- _Tree-Walking_ é uma variante de interpretador que executa a _AST_ diretamente, tornando o processo mais simples de implementar.

- Pode ser separado em quatro fases: _análise léxica_, _análise sintática_, _análise semântica_ e _interpretação_.

#pagebreak()

#figura_legendada(
	image("../../images/mapa_interpretador.png", height: 76%),
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

=== Análise Semântica

#figura_legendada(
	image("../../images/analise_semantica.png", height: 80%),
	[Fonte: baseado em #cite(<craftinginterpreters>, form: "prose").],
)

=== Interpretação

#figura_legendada(
	image("../../images/interpretacao.png", height: 80%),
	[Fonte: baseado em #cite(<craftinginterpreters>, form: "prose").],
)

= Tecnologias

== Rust e seu Ecossistema

=== Motivação

- *Tipagem forte:* gera garantias em tempo de compilação. Uso de enum e match é ótimo na manipulação dos tokens.

- *Tratamento de erros explícito:* toda etapa de um interpretador está sujeita a erros — tratamento explícito garante que os erros sejam tratados de acordo.

- *Alto desempenho:* desempenho anda lado a lado com ECS e interpretadores.

== Logos

Biblioteca de análise léxica baseada em _regex_ para Rust.

=== Motivação

Atualmente, Logos é uma das bibliotecas mais maturas e completas para análise léxica na linguagem Rust, além de ter alta compatibilidade com _Chumsky_.

#pagebreak()

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

Biblioteca de análise sintática baseada no paradigma declarativo para Rust.

=== Motivação

A escolha se deve aos mesmos motivos que levaram à escolha de Logos: maturidade e compatibilidade.

#pagebreak()

=== AST e _Parser_ para uma Linguagem de Aritmética com Chumsky

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
