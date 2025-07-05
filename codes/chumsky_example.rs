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
	// |- (*)
	// |  |- (-)
	// |  |  \- 2
	// |  \- 3
	// \- 5
	parser().parse("-2 * 3 + 5");
}