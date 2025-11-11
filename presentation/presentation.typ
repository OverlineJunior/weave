#import "style.typ": style

#let title = "Implementação de um Protótipo para uma Linguagem de Programação Orientada ao Entity Component System"
#let student = "Francisco Sebastiany Junior"
#let advisor = "Prof. Me. Luciano Santos Cardoso"

#show: style.with(title, student, advisor)

// ≈ 4 minutos
#include "chapters/introducao.typ"

// ≈ 8 minutos
#include "chapters/fundamentacao.typ"

// ≈ 4 minutos
#include "chapters/metodologia.typ"

// ≈ 8 minutos
#include "chapters/desenvolvimento.typ"

// ≈ 4 minutos
#include "chapters/resultados.typ"

// ≈ 2 minutos
// Demonstração do protótipo

= Obrigado!

= Referências

#pagebreak()

#bibliography(
	"./bibliography.yml",
	title: none,
	style: "associacao-brasileira-de-normas-tecnicas",
)
