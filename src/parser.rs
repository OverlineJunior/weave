use crate::{
    lexer::token::Token,
    parser::{expr::Expr, stmt::Stmt},
};
use chumsky::{input::ValueInput, prelude::*};

pub mod expr;
pub mod stmt;

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

    let expr = recursive(|expr| {
        let field = id.then_ignore(just(Token::Colon)).then(expr);

        let field_list = comma_separated(field).boxed();

        let atom = select! {
            Token::Int(value) => Expr::Literal(value),
            Token::String(value) => Expr::Literal(value),
        };

        let comp_cons = id
            .then_ignore(just(Token::LBrace))
            .then(field_list)
            .then_ignore(just(Token::RBrace))
            .map(|(name, fields)| Expr::ComponentCons { name, fields });

        let comp_list = comma_separated(comp_cons.clone()).boxed();

        let entity_cons = just(Token::Entity)
            .then_ignore(just(Token::LParen))
            .then(comp_list)
            .then_ignore(just(Token::RParen))
            .map(|(_, comps)| Expr::EntityCons(comps));

        atom.or(comp_cons).or(entity_cons)
    });

    let stmt = {
        let field_decl = id;

        let field_decl_list = comma_separated(field_decl);

        let comp_def = just(Token::Component)
            .ignore_then(id)
            .then_ignore(just(Token::LBrace))
            .then(field_decl_list)
            .then_ignore(just(Token::RBrace))
            .map(|(name, field_decls)| Stmt::ComponentDef { name, field_decls });

        let expr_stmt = expr.map(Stmt::Expr);

        comp_def.or(expr_stmt)
    };

    let program = stmt.repeated().collect::<Vec<Stmt>>().map(Stmt::Block);

    program
}

fn comma_separated<'a, I, F, O>(
    parser: F,
) -> impl Parser<'a, I, Vec<O>, extra::Err<Rich<'a, Token>>>
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
    F: Parser<'a, I, O, extra::Err<Rich<'a, Token>>>,
{
    parser
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .at_least(1)
        .collect::<Vec<_>>()
}
