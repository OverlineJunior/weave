#import "../style.typ": *

// ≈ 3-4 minutos

= Metodologia

== Características da Pesquisa

#grid(
  columns: (1fr, 1fr),
  [
    - Pesquisa *aplicada* e *exploratória*;

    - Abordagem *qualitativa*;

    - Contexto em *Linguagens de Programação ∩ ECS*.
  ],
  figura_legendada(
    image("../../images/interseccao_ecs_linguagem.png"),
    [Fonte: elaboração própria.],
  ),
)

== População e Amostra

- *População*: linguagens de programação e bibliotecas que implementam o ECS;

- *Amostra*: Rust, Jlox, Flecs e Bevy.

== Técnicas

=== Coleta de Dados

- Bibliográfica;
- Documental;
- Experimental.

#pause

=== Análise de Dados

- Documental;
- Bibliográfica;
- Comparativa.

== Procedimentos Metodológicos

+ *Visão Geral da Solução*: familiarizar o leitor com o escopo do trabalho;

+ *Planejamento*: definição das características da linguagem;

+ *Implementação*: organizada com base nas fases tradicionais de um interpretador _tree-walking_.
