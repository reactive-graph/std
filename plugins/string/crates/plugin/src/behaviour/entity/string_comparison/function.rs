use crate::model_string::NAMESPACE_STRING;
use crate::reactive::behaviour_functions;

pub type StringComparisonFunction = fn(String, String) -> bool;

pub const FN_CONTAINS: StringComparisonFunction = |lhs, rhs| lhs.contains(rhs.as_str());
pub const FN_ENDS_WITH: StringComparisonFunction = |lhs, rhs| lhs.ends_with(rhs.as_str());
pub const FN_STARTS_WITH: StringComparisonFunction = |lhs, rhs| lhs.starts_with(rhs.as_str());

behaviour_functions!(
    STRING_COMPARISONS,
    StringComparisonFunction,
    NAMESPACE_STRING,
    ("contains", FN_CONTAINS),
    ("ends_with", FN_ENDS_WITH),
    ("starts_with", FN_STARTS_WITH)
);
