#import "style.typ": style

#let title = "Implementação de um Protótipo para uma Linguagem de Programação Orientada ao Entity Component System"
#let student = "Francisco Sebastiany Junior"
#let advisor = "Prof. Me. Luciano Santos Cardoso"

#show: style.with(title, student, advisor)

#include "chapters/introducao.typ"

#include "chapters/fundamentacao.typ"

#include "chapters/metodologia.typ"

= Obrigado!

= Referências

#pagebreak()

#bibliography(
	"./bibliography.yml",
	title: none,
	style: "associacao-brasileira-de-normas-tecnicas",
)
