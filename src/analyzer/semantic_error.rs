use thiserror::Error;

use crate::analyzer::r#type::Type;

#[derive(Clone, PartialEq, Debug, Error)]
pub enum SemanticError {
	#[error("[line {line}] Undefined type: `{ty_name}`")]
	UndefinedType { ty_name: String, line: usize },
	#[error("[line {line}] Expected type `{expected}`, found `{found}`")]
	TypeMismatch {
		expected: Type,
		found: Type,
		line: usize,
	},
}
