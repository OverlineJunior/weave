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

- Pesquisa bibliográfica;

- Pesquisa documental;

- Pesquisa experimental.

#pause

=== Análise de Dados

- Análise documental e bibliográfica;

- Análise comparativa.

== Procedimentos Metodológicos

+ *Visão Geral da Solução*: planejamento macro;

+ *Planejamento*: planejamento micro;

+ *Implementação*: organizada com base nas fases tradicionais de um interpretador _tree-walking_.
