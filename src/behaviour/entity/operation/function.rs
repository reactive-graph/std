use lazy_static::lazy_static;
use std::collections::HashMap;
use voca_rs::case;

pub type StringOperationFunction = fn(String) -> String;

pub const FN_CAMEL_CASE: StringOperationFunction = |lhs: String| case::camel_case(lhs.as_str());
pub const FN_CAPITALIZE: StringOperationFunction = |lhs: String| case::capitalize(lhs.as_str(), true);
pub const FN_DECAPITALIZE: StringOperationFunction = |lhs: String| case::decapitalize(lhs.as_str(), true);
pub const FN_KEBAB_CASE: StringOperationFunction = |lhs: String| case::kebab_case(lhs.as_str());
pub const FN_LOWERCASE: StringOperationFunction = |lhs: String| lhs.to_lowercase();
pub const FN_LOWER_FIRST: StringOperationFunction = |lhs: String| case::lower_first(lhs.as_str());
pub const FN_PASCAL_CASE: StringOperationFunction = |lhs: String| case::pascal_case(lhs.as_str());
pub const FN_SHOUTY_KEBAB_CASE: StringOperationFunction = |lhs: String| case::shouty_kebab_case(lhs.as_str());
pub const FN_SHOUTY_SNAKE_CASE: StringOperationFunction = |lhs: String| case::shouty_snake_case(lhs.as_str());
pub const FN_SNAKE_CASE: StringOperationFunction = |lhs: String| case::snake_case(lhs.as_str());
pub const FN_SWAP_CASE: StringOperationFunction = |lhs: String| case::swap_case(lhs.as_str());
pub const FN_TITLE_CASE: StringOperationFunction = |lhs: String| case::title_case(lhs.as_str());
pub const FN_TRAIN_CASE: StringOperationFunction = |lhs: String| case::train_case(lhs.as_str());
pub const FN_TRIM: StringOperationFunction = |lhs: String| String::from(lhs.trim());
pub const FN_TRIM_START: StringOperationFunction = |lhs: String| String::from(lhs.trim_start());
pub const FN_TRIM_END: StringOperationFunction = |lhs: String| String::from(lhs.trim_end());
pub const FN_UPPERCASE: StringOperationFunction = |lhs: String| lhs.to_uppercase();
pub const FN_UPPER_FIRST: StringOperationFunction = |lhs: String| case::upper_first(lhs.as_str());

lazy_static! {
    pub static ref STRING_OPERATIONS: HashMap<&'static str, StringOperationFunction> = vec![
        ("camel_case", FN_CAMEL_CASE),
        ("capitalize", FN_CAPITALIZE),
        ("decapitalize", FN_DECAPITALIZE),
        ("kebab_case", FN_KEBAB_CASE),
        ("lowercase", FN_LOWERCASE),
        ("lower_first", FN_LOWER_FIRST),
        ("pascal_case", FN_PASCAL_CASE),
        ("shouty_kebab_case", FN_SHOUTY_KEBAB_CASE),
        ("shouty_snake_case", FN_SHOUTY_SNAKE_CASE),
        ("snake_case", FN_SNAKE_CASE),
        ("swap_case", FN_SWAP_CASE),
        ("title_case", FN_TITLE_CASE),
        ("train_case", FN_TRAIN_CASE),
        ("trim", FN_TRIM),
        ("trim_start", FN_TRIM_START),
        ("trim_end", FN_TRIM_END),
        ("uppercase", FN_UPPERCASE),
        ("upper_first", FN_UPPER_FIRST),
    ]
    .into_iter()
    .collect();
}
