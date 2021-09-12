use serde_json::Value;
use std::collections::HashMap;

pub type ComparisonGateFunction = fn(Value, Value) -> bool;

pub const FN_EQUALS: ComparisonGateFunction = |lhs, rhs| lhs == rhs;
pub const FN_NOT_EQUALS: ComparisonGateFunction = |lhs, rhs| lhs != rhs;

lazy_static! {
    pub static ref COMPARISON_GATES: HashMap<&'static str, ComparisonGateFunction> =
        vec![("equals", FN_EQUALS), ("not_equals", FN_NOT_EQUALS),]
            .into_iter()
            .collect();
}
