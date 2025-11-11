#import "../style.typ": figura_legendada
#import "@preview/touying:0.6.1": *

// ≈ 3-4 minutos

= Introdução e Motivação

---

// Explicar superficialmente o ECS antes da definição formal.
O *_Entity Component System_* (ECS) é um padrão de arquitetura com origens no desenvolvimento de jogos @ecsfaq;

#pause
#line(length: 100%)

Por mais que já usado fora de jogos, sua implementação é majoritariamente feita via *bibliotecas* ou *_frameworks_* @arewegameyet;

#pause
#line(length: 100%)

Por isso, este trabalho explora a implementação de uma *linguagem de programação* que torna o ECS um conceito de primeira classe.

---

= Objetivos

---

== Objetivo

Projetar e documentar um protótipo de interpretador para uma linguagem de programação orientada ao ECS.

---

== Objetivos Específicos

- Implementar as fases de análise léxica e sintática do interpretador a fim de tornar o ECS primeira classe na linguagem;

#pause

- Especificar conceitualmente a fase de interpretação do interpretador (mapeamento AST -> Flecs);

#pause

- Levantar decisões de implementação e desafios encontrados para auxiliar trabalhos futuros;

#pause

- Identificar limitações do protótipo e propor oportunidades de continuidade.

---
