use serde_json::Value;
use std::collections::HashMap;

pub type ComparisonGateFunction = fn(Value, Value) -> bool;

pub const FN_EQUALS: ComparisonGateFunction = |lhs, rhs| lhs == rhs;
pub const FN_NOT_EQUALS: ComparisonGateFunction = |lhs, rhs| lhs != rhs;
pub const FN_GREATER_THAN: ComparisonGateFunction = |lhs, rhs| {
    if lhs.is_number() && rhs.is_number() {
        return lhs.as_f64().unwrap() > rhs.as_f64().unwrap();
    }
    false
};
pub const FN_GREATER_THAN_OR_EQUALS: ComparisonGateFunction = |lhs, rhs| {
    if lhs.is_number() && rhs.is_number() {
        return lhs.as_f64().unwrap() >= rhs.as_f64().unwrap();
    }
    false
};
pub const FN_LOWER_THAN: ComparisonGateFunction = |lhs, rhs| {
    if lhs.is_number() && rhs.is_number() {
        return lhs.as_f64().unwrap() < rhs.as_f64().unwrap();
    }
    false
};
pub const FN_LOWER_THAN_OR_EQUALS: ComparisonGateFunction = |lhs, rhs| {
    if lhs.is_number() && rhs.is_number() {
        return lhs.as_f64().unwrap() <= rhs.as_f64().unwrap();
    }
    false
};

lazy_static! {
    pub static ref COMPARISON_GATES: HashMap<&'static str, ComparisonGateFunction> = vec![
        ("equals", FN_EQUALS),
        ("not_equals", FN_NOT_EQUALS),
        ("greater_than", FN_GREATER_THAN),
        ("greater_than_or_equals", FN_GREATER_THAN_OR_EQUALS),
        ("lower_than", FN_LOWER_THAN),
        ("lower_than_or_equals", FN_LOWER_THAN_OR_EQUALS),
    ]
    .into_iter()
    .collect();
}
