use crate::{
    lexer::token::Token,
    parser::{expr::Expr, stmt::Stmt},
};
use chumsky::{input::ValueInput, prelude::*};

pub mod expr;
pub mod stmt;

/*
Grammar (EBNF):

int = ? parsed by lexer ? ;
string = ? parsed by lexer ? ;
id = ? parsed by lexer ? ;

literal = int | string ;

field_decl = id ;
field_decl_list = field_decl , { "," , field_decl } , [ "," ] ;

data_field = id , ":" , expr ;
data_field_list = data_field , { "," , data_field } , [ "," ] ;

comp_decl = "component" , id , "{" , field_decl_list , "}" ;

comp_cons = id , "{" , data_field_list , "}" ;

entity_cons = "entity" , "(" , expr_list , ")" ;
expr_list = expr , { "," , expr } , [ "," ] ;

field_get = id , "." , id ;

var = id ;

expr = literal | comp_cons | entity_cons | field_get | var ;

expr_stmt = expr ;

var_decl = "var" , id , "=" , expr ;

print = "print" , expr ;

block = "{" , { stmt } , "}" ;

query_item = id , ":" , id ;
query = query_item , { "," , query_item } , [ "," ] ;

system_decl = "system" , id , "(" , query , ")" , block ;

stmt = comp_decl | expr_stmt | system_decl | var_decl | print | block ;

program = { stmt } ;
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
        let field = id.then_ignore(just(Token::Colon)).then(expr.clone());

        let field_list = comma_separated(field).boxed();

        let literal = select! {
            Token::Int(value) => Expr::Literal(value),
            Token::String(value) => Expr::Literal(value),
        };

        let comp_cons = id
            .then_ignore(just(Token::LBrace))
            .then(field_list)
            .then_ignore(just(Token::RBrace))
            .map(|(type_name, fields)| Expr::ComponentCons { type_name, fields });

        let entity_cons = just(Token::Entity)
            .then_ignore(just(Token::LParen))
            .then(comma_separated(expr.clone()).boxed())
            .then_ignore(just(Token::RParen))
            .map(|(_, comps)| Expr::EntityCons(comps));

        let var = id.map(|name| Expr::Var { name });

        let field_lhs = comp_cons.clone().or(var.clone());
        let comp_field_get = field_lhs
            .then_ignore(just(Token::Dot))
            .then(id)
            .map(|(lhs, field_name)| Expr::ComponentFieldGet { lhs: Box::new(lhs), field_name });

        literal.or(comp_cons).or(entity_cons).or(comp_field_get).or(var)
    });

    let stmt = recursive(|stmt| {
        let field_decl = id;

        let field_decl_list = comma_separated(field_decl);

        let comp_decl = just(Token::Component)
            .ignore_then(id)
            .then_ignore(just(Token::LBrace))
            .then(field_decl_list)
            .then_ignore(just(Token::RBrace))
            .map(|(name, field_decls)| Stmt::ComponentDecl { name, field_decls });

        let expr_stmt = expr.clone().map(Stmt::Expr);

        let block = just(Token::LBrace)
            .ignore_then(stmt.repeated().collect::<Vec<_>>())
            .then_ignore(just(Token::RBrace))
            .map(Stmt::Block)
            .boxed();

        let query_item = id.then_ignore(just(Token::Colon)).then(id);

        let query = comma_separated(query_item).boxed();

        let system_decl = just(Token::System)
            .ignore_then(id)
            .then_ignore(just(Token::LParen))
            .then(query)
            .then_ignore(just(Token::RParen))
            .then(block.clone())
            .map(|((name, query), body)| Stmt::SystemDecl {
                name,
                query,
                body: Box::new(body),
            });

        let var_decl = just(Token::Var)
            .ignore_then(id)
            .then_ignore(just(Token::Assign))
            .then(expr.clone())
            .map(|(name, value)| Stmt::VarDecl { name, value });

        // let print = just(Token::Print)
        //     .ignore_then(expr.clone())
        //     .map(Stmt::Print);

        let print = just(Token::Print)
            .ignore_then(just(Token::LParen))
            .ignore_then(comma_separated(expr))
            .then_ignore(just(Token::RParen))
            .map(Stmt::Print);

        comp_decl.or(expr_stmt).or(system_decl).or(var_decl).or(print).boxed()
    });

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
