use serde_json::json;
use serde_json::Value;

use crate::model_comparison::NAMESPACE_COMPARISON;
use crate::reactive::behaviour_functions;

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

behaviour_functions!(
    COMPARISON_GATES,
    ComparisonGateFunction,
    NAMESPACE_COMPARISON,
    ("equals", FN_EQUALS),
    ("not_equals", FN_NOT_EQUALS),
    ("greater_than", FN_GREATER_THAN),
    ("greater_than_or_equals", FN_GREATER_THAN_OR_EQUALS),
    ("lower_than", FN_LOWER_THAN),
    ("lower_than_or_equals", FN_LOWER_THAN_OR_EQUALS)
);
