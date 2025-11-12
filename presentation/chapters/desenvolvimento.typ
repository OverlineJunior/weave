#import "../style.typ": *

= Desenvolvimento

---

== Visão Geral

=== Sintaxe Alvo

#figura_legendada(
  ```rs
  component Person
  component Rock
  component Name { value }

  entity(Person, Name { value: "John Doe" })
  entity(Rock, Name { value: "Granite" })

  system greet_people(name: Name, Person) {
      print("Hello, ", name.value, "!")
  }
  ```,
  [],
)

=== Diagrama de Atividades

#figura_legendada(
  image("../../diagrams/diagrama_atividades.png", height: 82%),
  [Fonte: elaboração própria com PlantUML.],
)

=== Diagrama de Componentes

#figura_legendada(
  image("../../diagrams/diagrama_componentes.png"),
  [Fonte: elaboração própria com PlantUML.],
)

== Análise Léxica

=== Definição de _Tokens_

#figura_legendada(
  ```rs
  enum Token {
    Int(Value),    // 42
    String(Value), // "foo"
    Id(String),    // foo
    // ...
    Entity,        // entity
    Component,     // component
    System,        // system
  }
  ```,
  [Fonte: simplificação da definição de _token_ na implementação da análise léxica.],
)

---

=== Definição de Valores

#figura_legendada(
  ```rs
  enum Value {
    Int(i64),                       // 42
    String(String),                 // "foo"
    Entity(u64),                    // entity(c1, c2)
    ComponentType { name: String }, // component Foo
    ComponentInst {                 // Foo { }
      fields: Vec<(String, Value)>
    },
  }
  ```,
  [Fonte: simplificação da definição de valor na implementação da análise léxica.],
)

== Análise Sintática

=== Definição de Expressões

#figura_legendada(
  ```rs
  enum Expr {
    Literal(Value),                // 42, "foo", ...
    EntityCons(Vec<Expr>),         // entity(c1, c2)
    ComponentCons {                // Foo { }
      name: String,
      fields: Vec<(String, Expr)>,
    },
  }
  ```,
  [Fonte: simplificação da definição de expressão na implementação da análise léxica.],
)

---

#align(center + horizon)[=== Definição da Gramática Formal]

---

=== Implementação do _Parser_

#figura_legendada(
  ```rs
  let field_list = comma_separa ted(field); // a: 1, b: 2

  let comp_cons = id                       // Comp1
    .then_ignore(just(Token::LBrace))      // {
    .then(field_list)                      // a: 1, b: 2
    .then_ignore(just(Token::RBrace))      // }
    .map(|( name , fields )| Expr::ComponentCons {
      name,
      fields
    });
  ```,
  [Fonte: simplificação do _parser_ implementado na análise sintática.],
)

== Interpretação

#align(center + horizon)[=== Componentes Estáticos vs. Dinâmicos]

---

=== Componentes Dinâmicos em Rust

#figura_legendada(
  ```rs
  let dyn_comp = world
    .component_untyped()
    .member(i32::id(), "value");

  unsafe {
    world
      .entity()
      .add_id_unchecked(dyn_comp);
  }
  ```,
  [Fonte: elaboração própria com base em #cite(form: "prose", <flecs>).],
)
