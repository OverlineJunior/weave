use crate::stmt::Field;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
	Int,
	String,
	Component { name: String, fields: Vec<Field<Type>> },
}
