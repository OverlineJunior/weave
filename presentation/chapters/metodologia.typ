#import "@preview/touying:0.6.1": *
#import themes.simple: *
#import "../style.typ": *

#show: simple-theme.with(aspect-ratio: "16-9")

#title-slide[
	== Design e Implementação de um Protótipo de Interpretador para uma Linguagem de Programação Orientada ao Entity Component System

	#v(2em)

	*Aluno* Francisco Sebastiany Junior

	*Orientador* Prof. Me. Luciano Santos Cardoso
]

= Metodologia

== Tipo e Abordagem de Pesquisa

- Pesquisa aplicada;

- Pesquisa exploratória;

- Abordagem qualitativa.

== Contexto

- Desenvolvimento de linguagens de programação;

- Padrão arquitetural _Entity Component System_ (ECS).

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

+ *Design da Linguagem:* como será a sintaxe, tipagem e tratamento de erros? Qual será o paradigma? Quais _features_ do ECS abstrair?
+ *Análise do Design:* quais problemas o design resolve? Quão possível é a implementação dele?
+ *Implementação do Interpretador:* qual será a metodologia de desenvolvimento? Como o ECS será representado internamente? Como será feito o _lexer_ e o _parser_?
+ *Análise do Interpretador:* ele pode cumprir com o design proposto? Quais foram as dificuldades enfrentadas?

= Fim
