enum Stmt {
    // E.g.: `{ stmt1 stmt2 }`
    Block(Vec<Stmt>),
		// E.g.: `component Color { r, g, b }`
    ComponentDecl {
        name: String,
        field_decls: Vec<String>,
    },
		// E.g.: `system move(pos: Position, vel: Velocity) { ... }`
    SystemDecl {
        name: String,
        query: Vec<QueryItem>,
        body: Box<Stmt>,
    },
		// E.g.: `print(42, 69)`
    Print(Vec<Expr>),
		// E.g.: `42`
    Expr(Expr),
}
