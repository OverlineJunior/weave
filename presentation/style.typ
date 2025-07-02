#import "@preview/touying:0.6.1": *
#import themes.simple: *

#let style(title, student, advisor, body) = [
  #show: simple-theme.with(aspect-ratio: "16-9")

  #show raw: set text(size: 18pt)

  #show figure.where(kind: raw): set align(left)

  #title-slide[
    #grid(
      columns: 1,
      rows: (1fr, 1fr, 1fr),
      align(top)[#grid(
        columns: (1fr, 5fr, 1fr),
        rows: 1,
        align(left)[#image("../images/udc_logo.jpeg", height: 51pt)],
        text(size: 0.8em)[
          CENTRO UNIVERSITÁRIO DINÂMICA DAS CATARATAS

          CURSO DE CIÊNCIA DA COMPUTAÇÃO
        ],
      )],
      align(horizon)[== #title],
      align(bottom + left)[#text(size: 0.8em)[
        *Aluno* #student

        *Orientador* #advisor
      ]],
    )
  ]

  #body
]

#let figura_legendada(corpo, ..legendas) = {
  let arg_num = legendas.pos().len()

  set figure(caption: none)
  set stack(dir: ttb, spacing: 1em)

  if arg_num == 1 {
    figure(stack(
      corpo,
      align(start, text(size: 10pt, legendas.at(0))),
    ))
  } else if arg_num == 2 {
    figure(stack(
      corpo,
      align(start, text(size: 10pt, legendas.at(0))),
      align(start, text(size: 10pt, legendas.at(1))),
    ))
  } else if arg_num == 3 {
    figure(stack(
      corpo,
      align(start, text(size: 10pt, legendas.at(0))),
      align(start, text(size: 10pt, legendas.at(1))),
      align(start, text(size: 10pt, legendas.at(2))),
    ))
  } else {
    panic("Número de legendas não suportado.")
  }
}

#let figura_legendada_titulada(titulo, corpo, ..legendas) = {
  grid(
		columns: 1,
		rows: (1fr, 10fr),
		align(center)[#titulo],
		figura_legendada(
			corpo,
			..legendas,
		),
	)
}
