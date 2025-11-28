// Estrutura responsável por guardar o estado do lexer, incluindo o código-fonte e o contador de linhas.
struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
	// Consome parte do código fonte e retorna o token encontrado.
	fn next_token(&mut self) -> Result<Option<Spanned<Token>>, LexicalError> {
		// Se não houver mais caracteres a serem lidos, retorne None.
		if self.chars.peek().is_none() {
			return Ok(None);
		}

		let token = match self.chars.peek().unwrap() {
			// Tokens com um único lexema.
			c if is_single_lexeme(*c) => self.next_single_lexeme(),

			// Inteiros.
			c if c.is_ascii_digit() => self.next_int()?,

			// Strings.
			''' => self.next_string()?,

			// Palavra-chave ou identificador.
			c if is_id_start(*c) => self.next_id(),

			// Pulo de nova linha e incremento do contador de linhas.
			'\n' => {
				self.chars.next();
				self.line += 1;
				return self.next_token();
			}

			// Pulo de espaços em branco.
			c if is_whitespace(*c) => {
				self.chars.next();
				return self.next_token();
			}

			// Nenhum padrão foi encontrado.
			c => return Err(LexicalError::UnexpectedChar { ch: *c, line: 1 }),
		};

		Ok(Some(token))
	}
}