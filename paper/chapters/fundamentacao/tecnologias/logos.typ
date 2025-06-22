#import "../../../abnt_udc.typ": figura_legendada

=== Biblioteca Logos

Logos é uma biblioteca de análise léxica para Rust. Ela consiste na definição de _tokens_ através de _macros_ e expressões regulares, tornando o código extremamente conciso.

#figura_legendada(
    [Análise léxica para uma calculadora usando a biblioteca Logos.],
    ```rust
    use logos::Logos;

    #[derive(Logos)]
    #[logos(skip r"[ \t\n]+")]
    enum Token {
        #[token("+")]
        Plus,

        #[token("-")]
        Minus,

        #[token("*")]
        Multiply,

        #[token("/")]
        Divide,

        #[token("(")]
        LParen,

        #[token(")")]
        RParen,

        #[regex("[0-9]+", |lex| lex.slice().parse::<isize>().unwrap())]
        Integer(isize),
    }

    fn main() {
        let mut lexer = Token::lexer("1 + 2 * (3 - 4)");

        while let Some(token) = lexer.next() {
            println!("{:?}", token);
        }
    }
    ```,
    [Fonte: adaptado de #cite(<logos>, form: "prose").],
)

Além da simplicidade na definição dos _tokens_, o analisador léxico gerado é extremamente rápido, como mostra o _benchmark_ no repositório oficial da biblioteca:

#figura_legendada(
    [_Benchmark_ da biblioteca Logos.],
    table(
        columns: (auto, auto),
        inset: 10pt,
        align: horizon,
        table.header([Teste], [_Benchmark_]),
        [Identificadores],
        [647 ns/iter (+/- 27) = 1204 MB/s],
        [Palavras-chave, operadores e pontuações],
        [2,054 ns/iter (+/- 78) = 1037 MB/s],
        [Strings],
        [553 ns/iter (+/- 34) = 1575 MB/s]
    ),
    [Fonte: #cite(<logos>, form: "prose").],
)

Por fim, o uso da biblioteca Logos estará na implementação de toda a análise léxica, evitando que tempo seja gasto na análise manual de cada _token_. A motivação para a escolha da biblioteca se deve à sua simplicidade e maturidade, assim minimizando o tempo de desenvolvimento e garantindo maior estabilidade.

