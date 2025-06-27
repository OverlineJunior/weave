#import "@preview/touying:0.6.1": *
#import themes.simple: *

#let style(title, student, advisor, body) = [
  #show: simple-theme.with(aspect-ratio: "16-9")

  #show raw: set text(size: 18pt)

  #show figure.where(kind: raw): set align(left)

  // #show bibliography: it => [
  //   #empty-slide[#it]
  // ]

  #title-slide[
    #text(size: 1.25em)[#title]

    #v(2em)

    *Aluno* #student

    *Orientador* #advisor
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
