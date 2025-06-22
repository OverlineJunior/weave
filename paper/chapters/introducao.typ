= Introdução

Nos últimos anos, as empresas têm adotado novas abordagens para o desenvolvimento de software, especialmente as metodologias ágeis. Sabendo que tais metodologias favorecem a flexibilidade ao invés de planejamento rígido, vem a necessidade de um modelo arquitetural que permite uma adaptação rápida a novas demandas @agile. Um dos modelos que atendem esses requisitos é o padrão arquitetural Entity Component System (ECS).

O padrão ECS surgiu na área de desenvolvimento de jogos, com um dos fatores sendo a alta necessidade de adaptação rápida na indústria. Por mais que ECS continue sendo majoritariamente aplicado em jogos, sua utilidade expande para qualquer aplicação que dependa fortemente de rápida iteração de desenvolvimento, flexibilidade ou performance @flightdynamics.

Frequentemente, o padrão ECS é abstraído em forma de biblioteca, em uma determinada linguagem de programação. Isso é devido ao fato de que a implementação do padrão, principalmente de forma eficiente, é composta de vários detalhes técnicos, como a organização dos dados na memória (Mertens, 2024). Muitas vezes, tais detalhes são irrelevantes para o desenvolvedor, e por isso eles são ocultados pela interface da biblioteca.

Dado isso, o projeto visa abstrair o padrão ECS através de um método diferente — um protótipo para uma linguagem de programação orientada a ECS. Será abordado desde as escolhas de design da linguagem até a implementação do interpretador, por fim examinando, principalmente de forma qualitativa, se foi possível alcançar os objetivos determinados.

== Pergunta Problema

Como projetar e desenvolver um protótipo de interpretador para uma linguagem de programação orientada a ECS, aproveitando suas vantagens nativas para uma melhor abstração do padrão?

== Objetivos

=== Objetivo Geral

Investigar como o design e a implementação de um protótipo de interpretador para uma linguagem de programação orientada a ECS podem ser efetuados.

=== Objetivos Específicos

+ Definir os requisitos e princípios de design da linguagem;
+ Implementar um protótipo de interpretador funcional;
+ Avaliar o impacto e a viabilidade do protótipo.

== Justificativa

Com o crescimento da adoção de metodologias ágeis pelas empresas, vem a demanda por arquiteturas que promovam flexibilidade no desenvolvimento de software @flightdynamics. O padrão ECS tem se destacado por atender tais demandas, especialmente nas áreas de desenvolvimento de jogos e simulações.

A maioria das abstrações feitas sobre ECS estão no formato de bibliotecas específicas para determinadas linguagens de programação, limitando a expressividade do desenvolvedor no processo de abstração.

Dado isso, este trabalho propõe uma rota alternativa: a criação do design e implementação de uma linguagem de programação orientada a ECS. Com a capacidade de moldar a linguagem diante dos requisitos específicos do padrão ECS, essa rota propõe investigar e implementar abstrações que são difíceis ou até mesmo impossíveis de serem aplicadas em bibliotecas.

#pagebreak()
