#import "style.typ": style
#import "../paper/data.typ": title, student, advisor

#show: style.with(title, student, advisor)

#include "chapters/introducao.typ"

#include "chapters/fundamentacao.typ"

#include "chapters/metodologia.typ"

= Referências

#pagebreak()

#bibliography(
	"../bibliography.yml",
	title: none,
	style: "associacao-brasileira-de-normas-tecnicas",
)
