use flecs_ecs::prelude::*;
use crate::{interpreter::{ecs::{UserComponentInst, UserEntity, UserWorld}, runtime_error::RuntimeError}, lexer::value::Value, parser::expr::Expr};

pub fn eval(expr: &Expr, ecs: &'static World) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Literal(v) => Ok(v.clone()),
        Expr::Var { name } => {
            let value = ecs
                .get_var(name)
                .ok_or(RuntimeError::UndefinedVariable {
                    name: name.clone(),
                    line: 555,
                })?;

            Ok(value.clone())
        }
        Expr::ComponentFieldGet { lhs, field_name } => {
            let lhs_value = eval(lhs, ecs)?;

            if let Value::Component(comp_inst) = lhs_value {
                comp_inst.fields
                    .into_iter()
                    .find(|(f_name, _)| f_name == field_name)
                    .map(|(_, v)| v)
                    .ok_or(RuntimeError::UndefinedField {
                        lhs: format!("{:?}", lhs),
                        rhs: field_name.clone(),
                        line: 555,
                    })
            } else {
                Err(RuntimeError::BadFieldLhs {
                    expected: "ComponentInst".into(),
                    found: format!("{:?}", lhs_value),
                    line: 555,
                })
            }
        }
        // ? OK?
        Expr::EntityCons(exprs) => {
            let values = exprs.iter().map(|e| eval(e, ecs)).collect::<Result<Vec<_>, _>>()?;
            let entity = ecs.entity_named("user_entity");

            values.into_iter().for_each(|value| {
                if let Value::Component(comp_inst) = value {
                    entity.set_user_comp(comp_inst);
                } else {
                    panic!("Expected ComponentInst, got {:?}", value);
                }
            });

            Ok(Value::Entity(entity))
        }
        // ? OK?
        Expr::ComponentCons { type_name, fields } => {
            let comp_type = ecs
                .get_comp_type(type_name)
                .ok_or(RuntimeError::UndefinedComponentType {
                    name: type_name.clone(),
                    line: 555,
                })?;

            // Validate fields against component type declaration
            for (f_name, f_value) in fields {
                if !comp_type.field_decls.contains(f_name) {
                    return Err(RuntimeError::UndefinedField {
                        lhs: type_name.clone(),
                        rhs: f_name.clone(),
                        line: 555
                    });
                }
            }

            let evaluated_fields = fields
                .iter()
                .map(|(f_name, f_expr)| eval(f_expr, ecs).map(|v| (f_name.clone(), v)))
                .collect::<Result<Vec<_>, _>>()?;

            let comp_inst = UserComponentInst {
                type_name: type_name.clone(),
                fields: evaluated_fields.clone(),
                entity: ecs.component_named::<UserComponentInst>(format!("user_component_inst({})", type_name).as_str()),
            };

            Ok(Value::Component(comp_inst))
        }
    }
}
