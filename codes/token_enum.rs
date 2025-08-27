enum Token {
    Int(Value),     // e.g.: `42`
    String(Value),  // e.g.: `"Hello, world!"`
    Id(String),     // e.g.: em `component Foo`, `Foo` é um Id.

    Entity,         // entity
    Component,      // component
    System,         // system
    Var,            // var
    Print,          // print

    LParen,         // (
    RParen,         // )
    LBrace,         // {
    RBrace,         // }
    Comma,          // ,
    Dot,            // .
    Colon,          // :
    Assign,         // =
}