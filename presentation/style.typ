

#let figura_legendada(corpo, ..legendas) = {
  let arg_num = legendas.pos().len()
	let text_size = 16pt

  set figure(caption: none)
  set stack(dir: ttb, spacing: 1em)

  if arg_num == 1 {
    figure(stack(
      corpo,
      align(start, text(size: text_size, legendas.at(0))),
    ))
  } else if arg_num == 2 {
    figure(stack(
      corpo,
      align(start, text(size: text_size, legendas.at(0))),
      align(start, text(size: text_size, legendas.at(1))),
    ))
  } else if arg_num == 3 {
    figure(stack(
      corpo,
      align(start, text(size: text_size, legendas.at(0))),
      align(start, text(size: text_size, legendas.at(1))),
      align(start, text(size: text_size, legendas.at(2))),
    ))
  } else {
    panic("Número de legendas não suportado.")
  }
}
