enum Value {
    Int(i64), // e.g.: `42`
    String(String), // e.g.: `"Hello, world!"`
    Entity(u64), // e.g.: em `var e = entity()`, `e` é uma Entity.
		// e.g.: `component Position { x, y }` declara um ComponentType.
		ComponentType { name: String, id: u64 },
		// e.g.: `Position { x: 10, y: 20 }` retorna um ComponentInst.
    ComponentInst { fields: Vec<(String, Value)> },
}