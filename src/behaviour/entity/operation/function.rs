use lazy_static::lazy_static;
use std::collections::HashMap;

pub type StringOperationFunction = fn(String) -> String;

pub const FN_TRIM: StringOperationFunction = |lhs: String| String::from(lhs.trim());
pub const FN_TRIM_START: StringOperationFunction = |lhs: String| String::from(lhs.trim_start());
pub const FN_TRIM_END: StringOperationFunction = |lhs: String| String::from(lhs.trim_end());
pub const FN_UPPERCASE: StringOperationFunction = |lhs: String| lhs.to_uppercase();
pub const FN_LOWERCASE: StringOperationFunction = |lhs: String| lhs.to_lowercase();

lazy_static! {
    pub static ref STRING_OPERATIONS: HashMap<&'static str, StringOperationFunction> = vec![
        ("trim", FN_TRIM),
        ("trim_start", FN_TRIM_START),
        ("trim_end", FN_TRIM_END),
        ("uppercase", FN_UPPERCASE),
        ("lowercase", FN_LOWERCASE),
    ]
    .into_iter()
    .collect();
}
