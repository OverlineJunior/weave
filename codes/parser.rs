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
            .map(|(name, fields)| Expr::ComponentCons { name, fields });

        let expr_list = comma_separated(expr.clone()).boxed();

        let entity_cons = just(Token::Entity)
            .then_ignore(just(Token::LParen))
            .then(expr_list)
            .then_ignore(just(Token::RParen))
            .map(|(_, comps)| Expr::EntityCons(comps));

        let var = id.map(|name| Expr::Var { name });

        let field_get = id
            .then_ignore(just(Token::Dot))
            .then(id)
            .map(|(name, field)| Expr::FieldGet { name, field });

        literal.or(comp_cons).or(entity_cons).or(field_get).or(var)
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

        let print = just(Token::Print)
            .ignore_then(expr.clone())
            .map(Stmt::Print);

        comp_decl.or(expr_stmt).or(system_decl).or(var_decl).or(print).boxed()
    });

    let program = stmt.repeated().collect::<Vec<Stmt>>().map(Stmt::Block);

    program
}