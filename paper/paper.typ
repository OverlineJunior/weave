#import "@preview/codly:1.3.0": codly-init
#import "abnt_udc.typ": abnt_udc, capa, folha_de_rosto, outline_manual
#import "data.typ": title, student, advisor, city, year

#show: abnt_udc
#show: codly-init.with()

#capa(student, title, city, year)

#folha_de_rosto(student, title, advisor, city, year)

#outline(title: "Lista de Figuras", target: figure.where(kind: image))

#outline(title: "Lista de Tabelas e Quadros", target: figure.where(kind: table))

#outline(title: "Lista de Códigos", target: figure.where(kind: raw))

#outline_manual(
	"Lista de Abreviaturas e Siglas",
	[*AST*: Árvore Sintática Abstrata (do inglês, _Abstract Syntax Tree_)],
	[*ECS*: _Entity Component System_],
	[*LSP*: _Language Server Protocol_],
	[*MVC*: _Model-View-Controller_],
)

#outline(title: "Sumário")

#include "chapters/introducao.typ"

#include "chapters/fundamentacao.typ"

#include "chapters/metodologia.typ"

#bibliography(
	"../bibliography.yml",
	title: "Referências",
	style: "associacao-brasileira-de-normas-tecnicas"
)
