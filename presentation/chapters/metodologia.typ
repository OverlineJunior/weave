#import "../style.typ": *

// ≈ 3-4 minutos

= Metodologia

== Tipo e Abordagem de Pesquisa

- Pesquisa aplicada;

- Pesquisa exploratória;

- Abordagem qualitativa.

== Contexto

#grid(
	columns: (1fr, 1fr),
	rows: 1,
	column-gutter: 30pt,
	[
		- Desenvolvimento de linguagens de programação;

		- Padrão arquitetural _Entity Component System_ (ECS).
	],
	figura_legendada(
		image("../../images/interseccao_ecs_linguagem.png"),
		[Fonte: elaboração própria.],
	),
)

== População e Amostra

- *População*: linguagens de programação e bibliotecas que implementam ou dão suporte ao ECS;

- *Amostra*: Rust, Flecs e Bevy.

== Técnicas de Coleta de Dados

- Pesquisa bibliográfica;

- Pesquisa documental;

- Pesquisa experimental.

== Técnicas de Análise de Dados

- Análise documental e bibliográfica;

- Análise comparativa.

== Procedimentos Metodológicos

+ *Design da Linguagem:* como será a sintaxe, tipagem e tratamento de erros? Qual será o paradigma?
#pause
+ *Análise do Design:* quais problemas o design resolve? Quão possível é a implementação dele?
#pause
+ *Implementação do Interpretador:* qual será a metodologia de desenvolvimento? Como o ECS será representado internamente? Como será feito o _lexer_ e o _parser_?
#pause
+ *Análise do Interpretador:* ele pode cumprir com o design proposto? Quais foram as dificuldades enfrentadas?
