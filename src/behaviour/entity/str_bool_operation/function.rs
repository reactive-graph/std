use lazy_static::lazy_static;
use std::collections::HashMap;
use voca_rs::query;

pub type StrBoolFunction = fn(String) -> bool;

pub const FN_IS_ALPHA: StrBoolFunction = |lhs: String| query::is_alpha(lhs.as_str());
pub const FN_IS_ALPHA_DIGIT: StrBoolFunction = |lhs: String| query::is_alphadigit(lhs.as_str());
pub const FN_IS_BLANK: StrBoolFunction = |lhs: String| query::is_blank(lhs.as_str());
pub const FN_IS_CAMEL_CASE: StrBoolFunction = |lhs: String| query::is_camel_case(lhs.as_str());
pub const FN_IS_CAPITALIZE: StrBoolFunction = |lhs: String| query::is_capitalize(lhs.as_str());
pub const FN_IS_DECAPITALIZE: StrBoolFunction = |lhs: String| query::is_decapitalize(lhs.as_str());
pub const FN_IS_DIGIT: StrBoolFunction = |lhs: String| query::is_digit(lhs.as_str());
pub const FN_IS_EMPTY: StrBoolFunction = |lhs: String| query::is_empty(lhs.as_str());
pub const FN_IS_KEBAB_CASE: StrBoolFunction = |lhs: String| query::is_kebab_case(lhs.as_str());
pub const FN_IS_LOWER_FIRST: StrBoolFunction = |lhs: String| query::is_lower_first(lhs.as_str());
pub const FN_IS_LOWERCASE: StrBoolFunction = |lhs: String| query::is_lowercase(lhs.as_str());
pub const FN_IS_NUMERIC: StrBoolFunction = |lhs: String| query::is_numeric(lhs.as_str());
pub const FN_IS_PASCAL_CASE: StrBoolFunction = |lhs: String| query::is_pascal_case(lhs.as_str());
pub const FN_IS_SHOUTY_KEBAB_CASE: StrBoolFunction = |lhs: String| query::is_shouty_kebab_case(lhs.as_str());
pub const FN_IS_SHOUTY_SNAKE_CASE: StrBoolFunction = |lhs: String| query::is_shouty_snake_case(lhs.as_str());
pub const FN_IS_SNAKE_CASE: StrBoolFunction = |lhs: String| query::is_snake_case(lhs.as_str());
pub const FN_IS_TITLE_CASE: StrBoolFunction = |lhs: String| query::is_title(lhs.as_str());
pub const FN_IS_TRAIN_CASE: StrBoolFunction = |lhs: String| query::is_train_case(lhs.as_str());
pub const FN_IS_UPPER_FIRST: StrBoolFunction = |lhs: String| query::is_upper_first(lhs.as_str());
pub const FN_IS_UPPERCASE: StrBoolFunction = |lhs: String| query::is_uppercase(lhs.as_str());

lazy_static! {
    pub static ref STR_BOOL_FUNCTIONS: HashMap<&'static str, StrBoolFunction> = vec![
        ("is_alpha", FN_IS_ALPHA),
        ("is_alpha_digit", FN_IS_ALPHA_DIGIT),
        ("is_blank", FN_IS_BLANK),
        ("is_camel_case", FN_IS_CAMEL_CASE),
        ("is_capitalize", FN_IS_CAPITALIZE),
        ("is_decapitalize", FN_IS_DECAPITALIZE),
        ("is_digit", FN_IS_DIGIT),
        ("is_empty", FN_IS_EMPTY),
        ("is_kebab_case", FN_IS_KEBAB_CASE),
        ("is_lower_first", FN_IS_LOWER_FIRST),
        ("is_lowercase", FN_IS_LOWERCASE),
        ("is_numeric", FN_IS_NUMERIC),
        ("is_pascal_case", FN_IS_PASCAL_CASE),
        ("is_shouty_kebab_case", FN_IS_SHOUTY_KEBAB_CASE),
        ("is_shouty_snake_case", FN_IS_SHOUTY_SNAKE_CASE),
        ("is_snake_case", FN_IS_SNAKE_CASE),
        ("is_title_case", FN_IS_TITLE_CASE),
        ("is_train_case", FN_IS_TRAIN_CASE),
        ("is_upper_first", FN_IS_UPPER_FIRST),
        ("is_uppercase", FN_IS_UPPERCASE),
    ]
    .into_iter()
    .collect();
}
