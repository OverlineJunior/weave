enum Expr {
    // E.g.: `42`, `"hello"`
    Literal(Value),
    // E.g.: `Position { x: 10, y: 20 }`
    ComponentCons { name: String, fields: Vec<Field> },
    // E.g.: `entity(Comp1 { ... }, Comp2 { ... })`
    EntityCons(Vec<Expr>),
    // E.g.: `position.x`
    FieldGet { name: String, field: String },
}
