#import "../style.typ": *

= Resultados

---

== Escolha da Linguagem de Implementação

=== Objetivo

Escolher uma linguagem que permita (a) abstrair as estruturas de dados do interpretador de forma expressiva e (b) que tenha um bom ecossistema de bibliotecas relacionadas.

Foi escolhida a linguagem Rust.

#pause

=== Impacto

- Permitiu abstrações expressivas e seguras com _Algebraic Data Types_;
- Porém, tornou a implementação da fase de interpretação mais complexa devido à tipagem estática.

---

== Estratégia de Análise Léxica

=== Objetivo

Definir uma estratégia para transformar o código-fonte em _tokens_. Idealmente, ela deve ser simples de implementar e permitir a definição de erros customizados.

Foi escolhido implementar um _lexer_ manualmente.

#pause

=== Impacto

- Permitiu definir erros customizados;
- Porém, tornou a implementação mais complexa.

---

== Estratégia de Análise Sintática

=== Objetivo

Definir uma estratégia para transformar _tokens_ em uma AST. Idealmente, ela deve ser simples de implementar e permitir a definição de erros customizados.

Foi escolhido usar a biblioteca Chumsky.

#pause

=== Impacto

- Em comparação com o _parser_ do Jlox, a implementação ficou mais simples e mais parecida com a gramática formal;
- Permitiu definir erros customizados.

== Estratégia de Interpretação

=== Objetivo

Definir uma estratégia para interpretar a AST, ligando-a à uma biblioteca de ECS.

#pause

=== Impacto

- Foi escolhido usar a biblioteca Flecs;
- Não foi possível implementar devido à tipagem estática do Rust e à complexidade da API do Flecs;
- Porém, a formalização da estratégia explica a lógica de interpretação com componentes dinâmicos.
