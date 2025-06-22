#import "../../../abnt_udc.typ": figura_legendada

=== Paradigma de Programação

Um paradigma de programação pode ser definido como um conjunto de princípios que orientam o desenvolvimento de software, e que, consequentemente, geram um software estruturado de uma maneira específica ao paradigma @programmingparadigmsionos.

Os paradigmas são organizados em categorias, podendo ser vistos como uma hierarquia, assim como a figura abaixo mostra:

#figura_legendada(
	[Paradigmas de programação organizados hierarquicamente.],
	image("../../../../images/hierarquia_paradigmas.png", height: 35%),
	[Fonte: adaptado de #cite(<whatsprogrammingparadigm>, form: "prose").],
)

Com base na figura, observa-se a relevância dos paradigmas imperativo e declarativo, já que todos os outros paradigmas derivam, em maior ou menor grau, de um desses dois. Por mais que paradigmas mais específicos possam tomar rumos inesperados, eles ainda terão algum grau de herança dos paradigmas imperativo ou declarativo.

Dada a abrangência dos paradigmas imperativo e declarativo, o conhecimento de ambos já se torna suficiente para que o leitor entenda a essência da maioria dos outros paradigmas. Assim, a seguir, eles serão abordados:

- Paradigma imperativo: descreve a aplicação em termos de instruções que alteram o estado do programa, linha por linha. O foco está _em como_ fazer algo, geralmente oferecendo maior controle e menor abstração. Exemplos de linguagens imperativas incluem C, Java e Python.

- Paradigma declarativo: descreve a aplicação em termos de declarações que expressam _o que_ o programa deve fazer, e não _em como_ fazer. Esta abordagem costuma ser mais abstrata, porém com menos controle. Exemplos de linguagens declarativas incluem SQL, Haskell e Lisp.

==== Entity Component System pode ser considerado um paradigma?

O padrão Entity Component System (ECS) será extremamente crucial em todas as fases do projeto, por isso, é importante entender em que definição ele se encaixa.

Com base na definição de paradigma dada pelo dicionário Merriam-Webster:

#quote[A philosophical and theoretical framework of a scientific school or discipline within which theories, laws, and generalizations and the experiments performed in support of them are formulated. @merriamwebster]

#par(first-line-indent: 0em)[o autor da biblioteca de ECS Flecs, #cite(<ecsparadigm>, form: "author"), defende que o ECS não é um paradigma:]

#quote[That doesn’t quite match ECS. Having deep thoughts about ECS is not the same as having a philosophical framework. Tutorials, documentation and blog posts do not amount to “theories, laws and generalizations”. It is safe to say that the theoretical foundation for ECS is janky at best @ecsparadigm.]

Seguindo a conclusão de #cite(<ecsparadigm>, form: "author"), o ECS não será tratado como um paradigma ao decorrer do projeto, mas sim apenas como um padrão de arquitetura.
