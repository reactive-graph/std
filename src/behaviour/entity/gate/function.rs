use lazy_static::lazy_static;
use std::collections::HashMap;

pub type StringGateFunction = fn(String, String) -> String;

pub const FN_CONCAT: StringGateFunction = |lhs, rhs| format!("{}{}", lhs, rhs);

lazy_static! {
    pub static ref STRING_GATES: HashMap<&'static str, StringGateFunction> = vec![("concat", FN_CONCAT)].into_iter().collect();
}
