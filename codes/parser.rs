// O tipo de retorno do parser parece estranho, mas tudo que ele diz é que o parser consome tokens do tipo `Token` e produz uma AST do tipo `Stmt`, além de fornecer informações de erro detalhadas em caso de falha na análise.
fn parser<'a, I>() -> impl Parser<'a, I, Stmt, extra::Err<Rich<'a, Token>>>
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let id = select! {
        Token::Id(id) => id,
    };

	// O macro `recursive` se torna necessário quando é preciso referenciar o parser dentro de sua própria definição.
    let expr = recursive(|expr| {
        // id , ":" , expr ;
        let field = id.then_ignore(just(Token::Colon)).then(expr.clone());

        // field { "," , field } , [ "," ] ;
        let field_list = comma_separated(field).boxed();

        // int | string ;
        let literal = select! {
            Token::Int(value) => Expr::Literal(value),
            Token::String(value) => Expr::Literal(value),
        };

        // id , "{" , field_list , "}" ;
        let comp_cons = id
            .then_ignore(just(Token::LBrace))
            .then(field_list)
            .then_ignore(just(Token::RBrace))
            .map(|(name, fields)| Expr::ComponentCons { name, fields });

        // expr , { "," , expr } , [ "," ] ;
        let expr_list = comma_separated(expr.clone()).boxed();

        // "entity" , "(" , expr_list , ")" ;
        let entity_cons = just(Token::Entity)
            .then_ignore(just(Token::LParen))
            .then(expr_list)
            .then_ignore(just(Token::RParen))
            .map(|(_, comps)| Expr::EntityCons(comps));

        // id ;
        let var = id.map(|name| Expr::Var { name });

        // id , "." , id ;
        let field_get = id
            .then_ignore(just(Token::Dot))
            .then(id)
            .map(|(name, field)| Expr::FieldGet { name, field });

        // expr = literal | comp_cons | entity_cons | field_get | var ;
        literal.or(comp_cons).or(entity_cons).or(field_get).or(var)
    });

    let stmt = recursive(|stmt| {
        // id ;
        let field_decl = id;

        // field_decl , { "," , field_decl } , [ "," ] ;
        let field_decl_list = comma_separated(field_decl);

        // "component" , id , "{" , field_decl_list , "}" ;
        let comp_decl = just(Token::Component)
            .ignore_then(id)
            .then_ignore(just(Token::LBrace))
            .then(field_decl_list)
            .then_ignore(just(Token::RBrace))
            .map(|(name, field_decls)| Stmt::ComponentDecl { name, field_decls });

        // expr ;
        let expr_stmt = expr.clone().map(Stmt::Expr);

        // "{" , { stmt } , "}" ;
        let block = just(Token::LBrace)
            .ignore_then(stmt.repeated().collect::<Vec<_>>())
            .then_ignore(just(Token::RBrace))
            .map(Stmt::Block)
            .boxed();

        // id , ":" , id ;
        let query_item = id.then_ignore(just(Token::Colon)).then(id);

        // query_item , { "," , query_item } , [ "," ] ;
        let query = comma_separated(query_item).boxed();

        // "system" , id , "(" , query , ")" , block ;
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

        // "var" , id , "=" , expr ;
        let var_decl = just(Token::Var)
            .ignore_then(id)
            .then_ignore(just(Token::Assign))
            .then(expr.clone())
            .map(|(name, value)| Stmt::VarDecl { name, value });

        // "print" , "(" , expr_list , ")" ;
        let print = just(Token::Print)
            .ignore_then(expr.clone())
            .map(Stmt::Print);

        // stmt = comp_decl | expr_stmt | system_decl | var_decl | print | block ;
        comp_decl.or(expr_stmt).or(system_decl).or(var_decl).or(print).boxed()
    });

    // { stmt } ;
    let program = stmt.repeated().collect::<Vec<Stmt>>().map(Stmt::Block);

    program
}