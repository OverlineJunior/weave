use crate::parser::expr::Expr;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Expr),
    ComponentDef { name: String, field_decls: Vec<String> },
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Block(stmts) => {
                write!(f, "Block(stmts: {:?})", stmts)
            }
            Stmt::Expr(expr) => {
                write!(f, "Expr(expr: {})", expr)
            }
            Stmt::ComponentDef { name, field_decls } => {
                write!(f, "ComponentDef(name: {}, field_decls: {:?})", name, field_decls)
            }
        }
    }
}
