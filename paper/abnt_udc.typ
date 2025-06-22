#let abnt_udc(body) = [
  #set page(
    paper: "a4",
    margin: (
      top: 3cm,
      bottom: 2cm,
      left: 3cm,
      right: 2cm,
    ),
  )

  #set heading(
    numbering: "1.1",
  )

  #show heading: it => [
    #set block(below: 1.5em, above: 1.5em)

    #if it.level == 1 [
      #set text(weight: "bold")
      #block(upper(it))
    ] else if it.level == 2 [
      #set text(weight: "bold")
      #block(it)
    ] else [
      #set text(weight: "regular")
      #block(it)
    ]
  ]

  #set text(
    font: "Arial",
    size: 12pt,
  )

  #set par(
    justify: true,
    first-line-indent:(amount: 1.25cm, all: true),
    leading: 1.5em,
  )

  // Título de outline.
  #show outline: it => [
    #show heading: set heading(outlined: false)
    #show heading: set align(center)

    #show heading: it => [
      #it
      #v(1em)
    ]

    #it

    #pagebreak()
  ]

  // Lista de itens.
  // Por algum motivo, não consegui fazer um selector só para os vários tipos de figura.
  #let entry_de_figura(entry) = link(
    entry.element.location(),
    entry.indented([*#entry.prefix().*], entry.inner()),
  )
  #show outline.where(target: selector(figure.where(kind: image))): it => {
    show outline.entry: it => entry_de_figura(it)
    it
  }
  #show outline.where(target: selector(figure.where(kind: table))): it => {
    show outline.entry: it => entry_de_figura(it)
    it
  }
  #show outline.where(target: selector(figure.where(kind: raw))): it => {
    show outline.entry: it => entry_de_figura(it)
    it
  }

  // Sumário.
  #show outline.where(target: selector(heading)): it => {
    show outline.entry.where(level: 1): it => {
      set block(above: 3em)
      upper(it)
    }

    show outline.entry.where(level: 1).or(outline.entry.where(level: 2)): it => {
      set text(weight: "bold")
      it
    }

    it
  }

  #show bibliography: it => [
    #show heading: set align(center)

    #show heading: it => [
      #it
      #v(1em)
    ]

    #it
  ]

  #show figure.where(kind: image): set figure(supplement: [Figura])
  #show figure.where(kind: table): set figure(supplement: [Tabela])
  #show figure.where(kind: raw): set figure(supplement: [Código])

  #set figure.caption(position: top, separator: [ — ])

  // Blocos de código podem atravessar páginas.
  #show figure.where(kind: raw): set block(breakable: true)

  #show raw: it => [
    #set text(font: "Cascadia Mono", ligatures: true)
    #it
  ]

  #show table: block.with(stroke: (top: 1pt, bottom: 1pt))

  #set table(stroke: (_, y) => if y == 0 { (bottom: 0.5pt) })

  #show table.cell.where(y: 0): strong

  #set quote(block: true)

  #show quote.where(block: true): it => {
    set text(size: 10pt)
    set par(leading: 1em)
    set block(above: 3em, below: 3em)
    set rect(width: 100%, stroke: none)

    grid(
      columns: (4cm, 1fr),
      rows: 1,
      rect[],
      rect[#it],
    )
  }

  #body
]

#let capa(autor, titulo, local, data) = [
  #set par(justify: false)
  #set text(size: 15pt, weight: "bold")
  #set align(center)
  #set rect(width: 100%, height: 100%, stroke: none)

  #grid(
    columns: 1,
    rows: (5fr, 3.75fr, 10fr, 2fr),
    rect[#upper("Centro Universitário Dinâmica das Cataratas\nCurso de Bacharelado em Ciência da Computação")],
    rect[#autor],
    rect[#titulo],
    rect[#align(bottom)[#text(size: 12pt)[ #upper(local) \ #data]]],
  )

  #pagebreak()
]

#let folha_de_rosto(autor, titulo, orientador, local, data) = [
  #set par(justify: false)
  #set text(size: 15pt, weight: "bold")
  #set align(center)
  #set rect(width: 100%, height: 100%, stroke: none)

  #grid(
    columns: 1,
    rows: (5fr, 3.75fr, 10fr, 2fr),
    rect[#autor],
    rect[#titulo],
    grid(
      columns: (1fr, 1fr),
      rows: 1,
      [],
      rect[#align(horizon + left)[#text(size: 10pt)[Plano do Projeto da Disciplina de Projeto de Graduação I do Curso de Bacharelado em Ciência da Computação, apresentado à UDC como requisito parcial para obtenção do título de bacharel em Ciência da Computação. \ \ Orientador: #orientador]]],
    ),
    rect[#align(bottom)[#text(size: 12pt)[ #upper(local) \ #data]]],
  )

  #pagebreak()
]

#let todo(texto) = [
  // Removido para entrega.
  // #set text(weight: "bold", fill: red.darken(25%))
  // TODO! #texto
]

#let figura_legendada(titulo, corpo, ..legendas) = {
  let arg_num = legendas.pos().len()

  set figure(caption: titulo)
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
