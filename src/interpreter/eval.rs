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
        Expr::ComponentFieldGet { lhs, field_name } => {
            let lhs_value = eval(lhs, env, ecs)?;

            if let Value::ComponentInst { type_name: _, fields, component: _ } = lhs_value {
                fields.into_iter()
                    .find(|(f_name, _)| f_name == field_name)
                    .map(|(_, v)| v)
                    .ok_or(format!("Field {} not found in component instance", field_name))
            } else {
                Err(format!("Expected ComponentInst on LHS of field access, got {:?}", lhs_value))
            }
        }
        // ? OK?
        Expr::EntityCons(exprs) => {
            let comp_insts = exprs.iter().map(|e| eval(e, env, ecs)).collect::<Result<Vec<_>, _>>()?;
            let entity = ecs.entity_named("user_entity");

            comp_insts.into_iter().for_each(|comp_inst| {
                if let Value::ComponentInst { type_name, fields, component } = comp_inst {
                    // Unnamed so it gets a random id.
                    let aux_entity = ecs.entity();
                    let instance = component.instance;
                    entity.set_id(component, (aux_entity, instance));
                } else {
                    panic!("Expected ComponentInst, got {:?}", comp_inst);
                }
            });

            Ok(Value::Entity(entity))
        }
        // ? OK?
        // TODO! Make sure type_name leads to a defined component type and fields match its definition.
        Expr::ComponentCons { type_name, fields } => {
            let evaluated_fields = fields
                .iter()
                .map(|(f_name, f_expr)| eval(f_expr, env, ecs).map(|v| (f_name.clone(), v)))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Value::ComponentInst {
                type_name: type_name.clone(),
                fields: evaluated_fields.clone(),
                component: UserComponent {
                    type_name: type_name.clone(),
                    fields: evaluated_fields,
                    instance: ecs.component_named::<UserComponent>(format!("user_component({})", type_name).as_str()),
                },
            })
        }
    }
}
