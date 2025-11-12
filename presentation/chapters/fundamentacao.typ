#import "../style.typ": figura_legendada, figura_legendada_titulada
#import "@preview/touying:0.6.1": *

// ≈ 7-9 minutos

= Fundamentação Teórica

== Padrão de _Design_ de _Software_

#grid(
	columns: (1fr, 1fr),
	rows: 1,
	column-gutter: 30pt,
	[Solução reutilizável para um problema recorrente no design de software.],
	figura_legendada_titulada(
		[Exemplo: _Observer_],
		image("../../images/observer.png", height: 62%),
		[Fonte: elaboração própria com base em #cite(form: "prose", <headfirstdesignpatterns>)],
	),
)

== Padrão de Arquitetura de _Software_

#grid(
	columns: (1fr, 1fr),
	rows: 1,
	column-gutter: 30pt,
	[Parecido com um padrão de design, mas focado na organização de toda a aplicação.],
	figura_legendada_titulada(
		[Exemplo: _Model-View-Controller_],
		image("../../images/mvc.png", height: 66%),
		[Fonte: elaboração própria com base em #cite(form: "prose", <mdnmvc>)],
	),
)

== Paradigma de Programação

Conjunto de conceitos e princípios que orientam o desenvolvimento de software.

#figura_legendada(
	image("../../images/hierarquia_paradigmas.png", height: 55%),
	[Fonte: elaboração própria com base em #cite(<whatsprogrammingparadigm>, form: "prose").]
)

== _Entity Component System_

- Padrão de arquitetura com origem na indústria de jogos;

- Visa separar dados e lógica, promovendo *flexibilidade* e *modularidade* @ecsfaq;

- Separa a aplicação em três elementos fundamentais: *entidades*, *componentes* e *sistemas* @ecsfaq.

---

#grid(
	columns: (1.13fr, 1fr),
	rows: 1,
	column-gutter: 30pt,
	[
		=== Os Três Elementos Fundamentais

		- *Entidade:* identificador único sem dados ou lógica;

		- *Componente:* estrutura de dados que representa um conceito;

		- *Sistema:* lógica que opera em um conjunto de entidades com componentes específicos @ecsfaq.
	],
	figura_legendada(
		image("../../images/diagrama_ecs.png", height: 76%),
		[Fonte: elaboração própria com base em #cite(<ecsfaq>, form: "prose").]
	)
)

---

=== Tudo é uma Entidade

- *Componentes* e *sistemas* são associados a entidades @bevy.

- Permite a atribuição de metadados aos componentes e sistemas da aplicação.

- Pode tornar a linguagem mais ortogonal.

---

=== Relacionamentos

Permite o relacionamento entre entidades e componentes, tornando possível a representação de estruturas mais complexas, como hierarquias.

#figura_legendada(
	image("../../images/relacionamento_entidades.png", height: 37%),
	[Fonte: elaborado com base em #cite(<entityrelationships>, form: "prose").],
)

== Interpretador _Tree-Walking_

- Programa que executa diretamente o código fonte de uma linguagem de programação, linha por linha.

#pause

- _Tree-Walking_ é uma variante de interpretador que executa a árvore de sintaxe (AST) diretamente.

#pause

- Pode ser separado em três fases: _análise léxica_, _análise sintática_, e _interpretação_ @craftinginterpreters.

---

#figura_legendada(
	image("../../images/mapa_interpretador.png", height: 76%),
	[Fonte: #cite(<craftinginterpreters>, form: "prose").],
)

=== Análise Léxica

#figura_legendada(
	image("../../images/analise_lexica.png", height: 80%),
	[Fonte: elaboração própria com base em #cite(<craftinginterpreters>, form: "prose").],
)

=== Análise Sintática

#figura_legendada(
	image("../../images/analise_sintatica.png", height: 80%),
	[Fonte: elaboração própria com base em #cite(<craftinginterpreters>, form: "prose").],
)

=== Interpretação

#figura_legendada(
	image("../../images/interpretacao.png", height: 80%),
	[Fonte: elaboração própria com base em #cite(<craftinginterpreters>, form: "prose").],
)

= Tecnologias

== Rust

Linguagem de programação estática, compilada e multiparadigma @rustbook.

=== Motivação

- *Tipagem forte:* gera garantias em tempo de compilação. Uso de `enum` e `match` é ótimo na definição e manipulação das estruturas de dados que compõe a AST.

- *Tratamento de erros explícito:* toda etapa de um interpretador está sujeita a erros — tratamento explícito garante que os erros sejam tratados de acordo.

// == Logos

// Biblioteca de análise léxica baseada em _regex_ para Rust.

// === Motivação

// Atualmente, Logos é uma das bibliotecas mais maturas e completas para análise léxica na linguagem Rust, além de ter alta compatibilidade com _Chumsky_.

// #pagebreak()

// === _Lexer_ para uma Linguagem de Aritmética com _Logos_

// #figura_legendada(
// 	```rust
// 	#[derive(Logos)]
// 	#[logos(skip r"[\s]+")]
// 	enum Token {
// 		#[token("-")]
// 		Sub,

// 		#[regex("[0-9]+", |lex| lex.slice().parse().unwrap())]
// 		Int(i64),
// 	}
// 	```,
// 	[Fonte: adaptado de #cite(<logos>, form: "prose").],
// )

== Chumsky

Biblioteca de análise sintática baseada no paradigma declarativo @chumsky.

=== Motivação

- Atualmente, Chumsky é uma das bibliotecas mais maturas e completas para análise sintática em Rust;

- Após a implementação, foi visto que gramáticas escritas com Chumsky são extremamente parecidas com gramáticas formais.

---

=== _Parser_ para Declaração de Componentes

#figura_legendada(
	```rust
	// component Animal { especie, idade }
	let comp_decl = just(Token::Component)
		.ignore_then(id)
		.then_ignore(just(Token::LBrace))
		.then(field_decl_list)
		.then_ignore(just(Token::RBrace))
		.map(|(name, field_decls)| Stmt::ComponentDecl {
			name,
			field_decls,
		});
	```,
	[Fonte: elaboração própria com base na implementação do interpretador.],
)
