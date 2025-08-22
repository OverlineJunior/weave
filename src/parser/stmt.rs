use crate::parser::expr::Expr;
use std::fmt::{self, Debug, Display, Formatter};

pub type QueryItem = (String, String);

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Expr),
    ComponentDecl { name: String, field_decls: Vec<String> },
    SystemDecl { name: String, query: Vec<QueryItem>, body: Box<Stmt> },
    VarDecl { name: String, value: Expr },
    Print(Expr),
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
            Stmt::ComponentDecl { name, field_decls } => {
                write!(f, "ComponentDecl(name: {}, field_decls: {:?})", name, field_decls)
            }
            Stmt::SystemDecl { name, query, body } => {
                write!(f, "SystemDecl(name: {}, query: {:?}, body: {:?})", name, query, body)
            }
            Stmt::VarDecl { name, value } => {
                write!(f, "VarDecl(name: {}, value: {})", name, value)
            }
            Stmt::Print(expr) => {
                write!(f, "Print(expr: {})", expr)
            }
        }
    }
}
