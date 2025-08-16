use crate::stmt::TypeField;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
	Int,
	String,
	Component { name: String, fields: Vec<TypeField<Type>> },
}
