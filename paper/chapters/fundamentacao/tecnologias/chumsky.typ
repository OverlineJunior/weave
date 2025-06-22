#import "../../../abnt_udc.typ": figura_legendada

=== Biblioteca Chumsky

Chumsky é uma biblioteca de análise sintática para Rust. Ela é baseada no conceito de _parser combinators_ #footnote[Um _parser combinator_ consiste na combinação de parsers mais simples para criar parsers mais complexos, assim como é de costume compor uma função maior de funções menores.], e permite que a definição de _parsers_ seja feita de forma declarativa. Seu escopo abrange tanto gramáticas livres de contexto quanto gramáticas sensíveis ao contexto.

Ao usar a biblioteca para construir um _parser_, nota-se a influência do paradigma funcional. Por mais que seja um paradigma mais incomum, seu uso na biblioteca torna o processo de construção do _parser_ muito parecido com a construção de uma gramática formal:

#figura_legendada(
    [_Parser_ para uma gramática de expressões aritméticas simples usando a biblioteca Chumsky.],
    ```rust
    use chumsky::prelude::*;

    // Nodos da AST, cuja estrutura é representada por expressões recursivas.
    enum Expr<'a> {
        Int(f64),
        Neg(Box<Expr<'a>>),
        Add(Box<Expr<'a>>, Box<Expr<'a>>),
        Sub(Box<Expr<'a>>, Box<Expr<'a>>),
        Mul(Box<Expr<'a>>, Box<Expr<'a>>),
        Div(Box<Expr<'a>>, Box<Expr<'a>>),
    }

    // Parser final composto por parsers menores (int, unary, product e sum), ou seja, um parser combinator.
    fn parser<'a>() -> impl Parser<'a, &'a str, Expr<'a>> {
        let op = |c| just(c).padded()

        // int -> regex([0-9]+)
        let int = text::int(10).map(|s: &str| Expr::Int(s.parse().unwrap()))

        // unary -> atom | '-' unary
        let unary = op('-')
            .repeated()
            .foldr(int, |_op, rhs| Expr::Neg(Box::new(rhs)));

        // product -> unary ( '*' unary | '/' unary )*
        let product = unary.foldl(
            choice((
                op('*').to(Expr::Mul as fn(_, _) -> _),
                op('/').to(Expr::Div as fn(_, _) -> _),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        // sum -> product ( '+' product | '-' product )*
        let sum = product.foldl(
            choice((
                op('+').to(Expr::Add as fn(_, _) -> _),
                op('-').to(Expr::Sub as fn(_, _) -> _),
            ))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        )

        sum
    }

    fn main() {
        // (+)
        //  ├──(*)
        //  │   ├──(-)
        //  │   │   └──2
        //  │   └──3
        //  └──5
        parser().parse("-2 * 3 + 5");
    }
    ```,
    [Fonte: adaptado de #cite(<chumsky>, form: "prose").],
)

De acordo com a classificação do _benchmark_ da biblioteca e seus competidores, localizada no repositório oficial, Chusmky tem a capacidade de ser a biblioteca de análise sintática mais rápida para Rust:

#figura_legendada(
    [Classificação do _benchmark_ da biblioteca Chumsky e competidores.],
    table(
        columns: 3,
        inset: 10pt,
        align: horizon,
        table.header([Classificação], [Biblioteca], [Tempo de Execução]),
        [1],
        [chumsky (check-only)],
        [140.77 µs],
        [2],
        [winnow],
        [178.91 µs],
        [3],
        [chusmky],
        [210.43 µs],
        [4],
        [sn],
        [237.94 µs],
        [5],
        [serde_json],
        [477.41 µs],
        [6],
        [nom],
        [526.52 µs],
        [7],
        [pest],
        [1.9706 ms],
        [8],
        [pom],
        [13.730 ms],
    ),
    [Fonte: #cite(<chumsky>, form: "prose").],
)

Por fim, o uso da biblioteca Chumsky estará na implementação da análise sintática, e será utilizada em conjunto com a biblioteca Logos na implementação do interpretador como um todo. Assim como foi o caso com Logos, a escolha de Chumsky se deve à sua maturidade.
