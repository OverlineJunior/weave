use crate::{
    stmt::{Field, Stmt},
    token::Token,
};
use chumsky::{input::ValueInput, prelude::*};

/*
comp_def = "component" , id , "{", field_list , "}" ;

field_list = field , ( "," , field )* ;

field = id , ":" , type ;
*/

#[allow(clippy::let_and_return)]
pub fn parser<'a, I>() -> impl Parser<'a, I, Stmt<()>, extra::Err<Rich<'a, Token>>>
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let id = select! {
        Token::Id(id) => id,
    };

    let field = id
        .then_ignore(just(Token::Colon))
        .then(id)
        .map(|(name, ty_name)| Field { name, ty_name, ty: () });

    let field_list = field
		.separated_by(just(Token::Comma))
		.allow_trailing()
		.at_least(1)
		.collect::<Vec<Field<()>>>();

	let comp_def = just(Token::Component)
		.ignore_then(id)
		.then_ignore(just(Token::LBrace))
		.then(field_list)
		.then_ignore(just(Token::RBrace))
		.map(|(name, fields)| Stmt::ComponentDef { name, fields });

	comp_def
}
