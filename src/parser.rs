use crate::{
    expr::{DataField, Expr},
    stmt::{Stmt, TypeField},
    token::Token,
};
use chumsky::{input::ValueInput, prelude::*};

/*
Grammar (EBNF):

int 			= ? parsed by lexer ? 							;

string			= ? parsed by lexer ? 							;

id 				= ? parsed by lexer ? 							;

atom 			= int | string 									;

type_field 		= id , ":" , id 								;

data_field 		= id , ":" , atom 								; // TODO! Replace atom with expr.

type_field_list = type_field , { "," , type_field } , [ "," ] 	;

data_field_list = data_field , { "," , data_field } , [ "," ] 	;

comp_def 		= "component" , id , "{", type_field_list , "}" ;

comp_cons 		= id , "{", data_field_list , "}" 				;

expr_stmt 		= atom | comp_cons 								;

program 		= { comp_def | expr_stmt } 						;
*/

#[allow(clippy::let_and_return)]
pub fn parser<'a, I>() -> impl Parser<'a, I, Stmt, extra::Err<Rich<'a, Token>>>
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let id = select! {
        Token::Id(id) => id,
    };

    let atom = select! {
        Token::Int(value) => Expr::Literal(value, ()),
        Token::String(value) => Expr::Literal(value, ()),
    };

    let type_field = id
        .then_ignore(just(Token::Colon))
        .then(id)
        .map(|(name, ty_name)| TypeField {
            name,
            ty_name,
            ty: (),
        });

    let data_field = id
        .then_ignore(just(Token::Colon))
        .then(atom)
        .map(|(name, atom)| DataField { name, data: atom });

    let type_field_list = field_list(type_field);

    let data_field_list = field_list(data_field);

    let comp_def = just(Token::Component)
        .ignore_then(id)
        .then_ignore(just(Token::LBrace))
        .then(type_field_list)
        .then_ignore(just(Token::RBrace))
        .map(|(name, fields)| Stmt::ComponentDef { name, fields });

    let comp_cons = id
        .then_ignore(just(Token::LBrace))
        .then(data_field_list)
        .then_ignore(just(Token::RBrace))
        .map(|(name, fields)| Expr::ComponentCons { name, fields });

    let expr_stmt = atom.or(comp_cons).map(Stmt::Expr);

    let program = comp_def
        .or(expr_stmt)
        .repeated()
        .collect::<Vec<Stmt>>()
        .map(Stmt::Block);

    program
}

fn field_list<'a, I, F, O>(field_kind: F) -> impl Parser<'a, I, Vec<O>, extra::Err<Rich<'a, Token>>>
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
    F: Parser<'a, I, O, extra::Err<Rich<'a, Token>>>,
{
    field_kind
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .at_least(1)
        .collect::<Vec<_>>()
}
