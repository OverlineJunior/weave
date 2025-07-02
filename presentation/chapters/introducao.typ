#import "../style.typ": figura_legendada
#import "@preview/touying:0.6.1": *

// ≈ 3-4 minutos

= Introdução e Motivação

#pagebreak()

#align(center)[#grid(
	columns: (1fr, 1fr),
	rows: (2em, 1fr),
	[Modelo Cascata],
	[Modelo Ágil],
	figura_legendada(
		image("../../images/modelo_cascata.png", height: 10em),
		[Fonte: elaboração própria.],
	),
	figura_legendada(
		image("../../images/modelo_agil.png", height: 10em),
		[Fonte: elaboração própria.],
	),
)]

#pagebreak()

// Explicar superficialmente o ECS antes da definição formal.
O padrão de arquitetura *_Entity Component System_* (ECS) é uma abordagem que se adequa ao modelo ágil por ser *flexível* e *adaptável*.

#pause
#line(length: 100%)

// Citar como o ECS costuma ser abstraído quase sempre como uma biblioteca.
Porém ainda não existem muitos estudos exploratórios sobre o tema. // Por ser um padrão relativamente novo e específico.

#pause
#line(length: 100%)

Com base nisso, este trabalho busca explorar o padrão ECS imbutido em uma *linguagem de programação*.

#pagebreak()

= Objetivos

#pagebreak()

== Objetivo Geral

#grid(
	columns: (1fr, 1fr),
	rows: 1,
	column-gutter: 30pt,
	[Explorar a viabilidade de projetar e implementar uma linguagem de programação orientada ao padrão ECS.],
	figura_legendada(
		image("../../images/interseccao_ecs_linguagem.png"),
		[Fonte: elaboração própria.],
	),
)

#pagebreak()

== Objetivos Específicos

+ Definir os requisitos e princípios de design da linguagem

+ Implementar um protótipo de interpretador funcional

+ Avaliar o impacto e a viabilidade do trabalho

#pagebreak()
