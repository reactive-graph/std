use std::sync::Arc;
use std::sync::LazyLock;

use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFactoryCreator;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctions;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctionsStorage;
use serde_json::Value;
use serde_json::json;

use reactive_graph_std_comparison_model::NAMESPACE_COMPARISON;

use crate::behaviour::entity::gate::behaviour::ComparisonGateFactory;

pub type ComparisonGateFunction = fn(&Value, &Value) -> Value;

pub const FN_EQUALS: ComparisonGateFunction = |lhs, rhs| json!(lhs == rhs);
pub const FN_NOT_EQUALS: ComparisonGateFunction = |lhs, rhs| json!(lhs != rhs);
pub const FN_GREATER_THAN: ComparisonGateFunction = |lhs, rhs| {
    if let Some(lhs) = lhs.as_f64() {
        if let Some(rhs) = rhs.as_f64() {
            return json!(lhs > rhs);
        }
    }
    if let Some(lhs) = lhs.as_i64() {
        if let Some(rhs) = rhs.as_i64() {
            return json!(lhs > rhs);
        }
    }
    if let Some(lhs) = lhs.as_u64() {
        if let Some(rhs) = rhs.as_u64() {
            return json!(lhs > rhs);
        }
    }
    Value::Bool(false)
};
pub const FN_GREATER_THAN_OR_EQUALS: ComparisonGateFunction = |lhs, rhs| {
    if let Some(lhs) = lhs.as_f64() {
        if let Some(rhs) = rhs.as_f64() {
            return json!(lhs >= rhs);
        }
    }
    if let Some(lhs) = lhs.as_i64() {
        if let Some(rhs) = rhs.as_i64() {
            return json!(lhs >= rhs);
        }
    }
    if let Some(lhs) = lhs.as_u64() {
        if let Some(rhs) = rhs.as_u64() {
            return json!(lhs >= rhs);
        }
    }
    Value::Bool(false)
};
pub const FN_LOWER_THAN: ComparisonGateFunction = |lhs, rhs| {
    if let Some(lhs) = lhs.as_f64() {
        if let Some(rhs) = rhs.as_f64() {
            return json!(lhs < rhs);
        }
    }
    if let Some(lhs) = lhs.as_i64() {
        if let Some(rhs) = rhs.as_i64() {
            return json!(lhs < rhs);
        }
    }
    if let Some(lhs) = lhs.as_u64() {
        if let Some(rhs) = rhs.as_u64() {
            return json!(lhs < rhs);
        }
    }
    Value::Bool(false)
};
pub const FN_LOWER_THAN_OR_EQUALS: ComparisonGateFunction = |lhs, rhs| {
    if let Some(lhs) = lhs.as_f64() {
        if let Some(rhs) = rhs.as_f64() {
            return json!(lhs <= rhs);
        }
    }
    if let Some(lhs) = lhs.as_i64() {
        if let Some(rhs) = rhs.as_i64() {
            return json!(lhs <= rhs);
        }
    }
    if let Some(lhs) = lhs.as_u64() {
        if let Some(rhs) = rhs.as_u64() {
            return json!(lhs <= rhs);
        }
    }
    Value::Bool(false)
};

const FACTORY_CREATOR: EntityBehaviourFactoryCreator<ComparisonGateFunction> = |ty, f| Arc::new(ComparisonGateFactory::new(ty.clone(), f));

pub static COMPARISON_GATES: EntityBehaviourFunctionsStorage<ComparisonGateFunction> = LazyLock::new(|| {
    EntityBehaviourFunctions::<ComparisonGateFunction>::with_namespace(NAMESPACE_COMPARISON, FACTORY_CREATOR)
        .behaviour("equals", FN_EQUALS)
        .behaviour("not_equals", FN_NOT_EQUALS)
        .behaviour("greater_than", FN_GREATER_THAN)
        .behaviour("greater_than_or_equals", FN_GREATER_THAN_OR_EQUALS)
        .behaviour("lower_than", FN_LOWER_THAN)
        .behaviour("lower_than_or_equals", FN_LOWER_THAN_OR_EQUALS)
        .get()
});
