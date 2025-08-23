use std::collections::HashMap;

use crate::{lexer::value::Value, parser::expr::Expr};

pub fn eval(expr: &Expr, env: &mut HashMap<String, Value>) -> Result<Value, String> {
    match expr {
        Expr::Literal(v) => Ok(v.clone()),
        Expr::Var { name } => {
            let value = env.get(name).ok_or(format!("Undefined variable: {}", name))?;
            Ok(value.clone())
        }
        Expr::FieldGet { name, field } => {
            // TODO! Lookup variable in environment and get field.
            Ok(Value::String("field_value".to_string()))
        }
		Expr::EntityCons(exprs) => {
			// TODO! Evaluate expressions and construct entity with id.
			Ok(Value::Entity(0))
		}
        Expr::ComponentCons { name, fields } => {
            let evaluated_fields = fields
                .iter()
                .map(|(f_name, f_expr)| eval(f_expr, env).map(|v| (f_name.clone(), v)))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Value::Component {
                name: name.clone(),
                fields: evaluated_fields,
            })
        }
    }
}
