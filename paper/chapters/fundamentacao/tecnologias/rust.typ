===	Linguagem de Programação Rust

A linguagem de programação utilizada para o desenvolvimento do interpretador será Rust. A motivação por trás da escolha se dá pelos seguintes fatos:

- Possui um sistema de tipagem forte — o uso de enum e match é especialmente útil na definição dos tokens e na construção da AST;
- O tratamento de erros é explícito, indicando com clareza quais partes do código precisam ser tratadas adequadamente. Todas as fases de um interpretador estão sujeitas a erros, e por isso, tratá-los do jeito mais claro possível é benéfico para o estudo do código;
- Possui alto desempenho, muitas vezes comparado ao de C. Desempenho é importante não só para ECS em si, mas também para qualquer interpretador, minimizando o tempo que o desenvolvedor espera pela execução de seu código @rustbook.
