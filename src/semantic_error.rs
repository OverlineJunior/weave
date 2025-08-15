use thiserror::Error;

#[derive(Clone, PartialEq, Debug, Error)]
pub enum SemanticError {
	#[error("[line {}] Undefined type: `{}`", line, ty_name)]
	UndefinedType { ty_name: String, line: usize },
}
