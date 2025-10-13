use std::collections::HashMap;
use flecs_ecs::prelude::*;
use crate::{interpreter::ecs::UserComponent, lexer::value::Value, parser::expr::Expr};

pub fn eval(
    expr: &Expr,
    env: &mut HashMap<String, Value>,
    ecs: &'static World
) -> Result<Value, String> {
    match expr {
        Expr::Literal(v) => Ok(v.clone()),
        Expr::Var { name } => {
            let value = env
                .get(name)
                .ok_or(format!("Undefined variable: {}", name))?;
            Ok(value.clone())
        }
        Expr::FieldGet { name, field } => {
            // TODO! Lookup variable in environment and get field.
            Ok(Value::String("field_value".to_string()))
        }
        // ? OK?
        Expr::EntityCons(exprs) => {
            // TODO! Evaluate expressions and construct entity with id.

            let comp_insts = exprs.iter().map(|e| eval(e, env, ecs)).collect::<Result<Vec<_>, _>>()?;
            let entity = ecs.entity_named("user_entity");

            comp_insts.into_iter().for_each(|comp_inst| {
                if let Value::ComponentInst { fields, component } = comp_inst {
                    // Unnamed so it gets a random id.
                    let aux_entity = ecs.entity();
                    let id = component.id;
                    entity.set_id(component, (aux_entity, id));
                } else {
                    panic!("Expected ComponentInst, got {:?}", comp_inst);
                }
            });

            Ok(Value::Entity(entity))
        }
        // ? OK?
        Expr::ComponentCons { name, fields } => {
            let evaluated_fields = fields
                .iter()
                .map(|(f_name, f_expr)| eval(f_expr, env, ecs).map(|v| (f_name.clone(), v)))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Value::ComponentInst {
                fields: evaluated_fields.clone(),
                component: UserComponent {
                    fields: evaluated_fields,
                    id: ecs.component_named::<UserComponent>(format!("user_component({})", name).as_str()),
                },
            })
        }
    }
}
