use lazy_static::lazy_static;
use std::collections::HashMap;

pub type StringComparisonFunction = fn(String, String) -> bool;

pub const FN_STARTS_WITH: StringComparisonFunction = |lhs, rhs| lhs.starts_with(rhs.as_str());
pub const FN_ENDS_WITH: StringComparisonFunction = |lhs, rhs| lhs.ends_with(rhs.as_str());
pub const FN_CONTAINS: StringComparisonFunction = |lhs, rhs| lhs.contains(rhs.as_str());

lazy_static! {
    pub static ref STRING_COMPARISONS: HashMap<&'static str, StringComparisonFunction> =
        vec![("starts_with", FN_STARTS_WITH), ("ends_with", FN_ENDS_WITH), ("contains", FN_CONTAINS),]
            .into_iter()
            .collect();
}
