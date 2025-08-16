use crate::{analyzer::semantic_error::SemanticError, lexer::lexical_error::LexicalError};

#[derive(Debug)]
pub enum Error {
    LexicalError(LexicalError),
    SemanticError(SemanticError),
}

impl From<LexicalError> for Error {
    fn from(error: LexicalError) -> Self {
        Error::LexicalError(error)
    }
}

impl From<SemanticError> for Error {
    fn from(error: SemanticError) -> Self {
        Error::SemanticError(error)
    }
}
