use thiserror::Error;

#[derive(Clone, PartialEq, Debug, Error)]
pub enum RuntimeError {
    #[error("[line {line}] Undefined variable `{name}`")]
    UndefinedVariable {
        name: String,
        line: usize,
    },

    #[error("[line {line}] `{lhs}` has no field `{rhs}`")]
    UndefinedField {
        lhs: String,
        rhs: String,
        line: usize,
    },

    #[error("[line {line}] Undefined component type `{name}`")]
    UndefinedComponentType {
        name: String,
        line: usize,
    },

    #[error("[line {line}] Expected `{expected}` on left-hand side of field access, found `{found}`")]
    BadFieldLhs {
        expected: String,
        found: String,
        line: usize,
    },

    #[error("[line {line}] Expected `{expected}`, found `{found}`")]
    ExpectedFound {
        expected: String,
        found: String,
        line: usize,
    },

    #[error("[line {line}] Cannot redeclare component `{name}`")]
    ComponentRedeclaration {
        name: String,
        line: usize,
    },
}
