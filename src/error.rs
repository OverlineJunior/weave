use crate::{lexer::lexical_error::LexicalError, semantic_error::SemanticError};

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
