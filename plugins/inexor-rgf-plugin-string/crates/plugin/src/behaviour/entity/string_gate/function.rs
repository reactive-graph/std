use voca_rs::chop;

use crate::model_string::NAMESPACE_STRING;
use crate::reactive::behaviour_functions;

pub type StringGateFunction = fn(String, String) -> String;

pub const FN_CHOP_AFTER: StringGateFunction = |lhs, rhs| chop::after(lhs.as_str(), rhs.as_str());
pub const FN_CHOP_AFTER_LAST: StringGateFunction = |lhs, rhs| chop::after_last(lhs.as_str(), rhs.as_str());
pub const FN_CONCAT: StringGateFunction = |lhs, rhs| format!("{}{}", lhs, rhs);
pub const FN_BEFORE: StringGateFunction = |lhs, rhs| chop::before(lhs.as_str(), rhs.as_str());
pub const FN_BEFORE_LAST: StringGateFunction = |lhs, rhs| chop::before_last(lhs.as_str(), rhs.as_str());
pub const FN_REMOVE_PREFIX: StringGateFunction = |lhs, rhs| chop::removeprefix(lhs.as_str(), rhs.as_str());
pub const FN_REMOVE_SUFFIX: StringGateFunction = |lhs, rhs| chop::removesuffix(lhs.as_str(), rhs.as_str());

behaviour_functions!(
    STRING_GATES,
    StringGateFunction,
    NAMESPACE_STRING,
    ("chop_after", FN_CHOP_AFTER),
    ("chop_after_last", FN_CHOP_AFTER_LAST),
    ("concat", FN_CONCAT),
    ("chop_before", FN_BEFORE),
    ("chop_before_last", FN_BEFORE_LAST),
    ("chop_remove_prefix", FN_REMOVE_PREFIX),
    ("chop_remove_suffix", FN_REMOVE_SUFFIX)
);
