#import "style.typ": *
#import "@preview/cades:0.3.1": qr-code

#let title = "Implementação de um Protótipo para uma Linguagem de Programação Orientada ao Entity Component System"
#let student = "Francisco Sebastiany Junior"
#let advisor = "Prof. Me. Luciano Santos Cardoso"

#show: style.with(title, student, advisor)

// ≈ 2 minutos
#include "chapters/introducao.typ"

// ≈ 8 minutos
#include "chapters/fundamentacao.typ"

// ≈ 2 minutos
#include "chapters/metodologia.typ"

// ≈ 8 minutos
#include "chapters/desenvolvimento.typ"

// ≈ 6 minutos
#include "chapters/resultados.typ"

// ≈ 2 minutos
#include "chapters/conclusao.typ"

// ≈ 2 minutos
// Demonstração do protótipo

#empty-slide[
	#set align(center + horizon)

	#text(size: 1.25em)[*Obrigado pela atenção!*]

	#v(1.5em)

	#qr-code("https://github.com/OverlineJunior/weave/blob/master/presentation/bibliography.yml")

	*Referências*
]

= Referências

#set text(size: 0.48em)

#bibliography(
	"./bibliography.yml",
	title: none,
	style: "associacao-brasileira-de-normas-tecnicas",
)
