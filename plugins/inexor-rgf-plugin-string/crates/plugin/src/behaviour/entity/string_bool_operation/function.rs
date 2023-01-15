use voca_rs::query;

use crate::model_string::NAMESPACE_STRING;
use crate::reactive::behaviour_functions;

pub type StringBoolFunction = fn(String) -> bool;

pub const FN_IS_ALPHA: StringBoolFunction = |lhs: String| query::is_alpha(lhs.as_str());
pub const FN_IS_ALPHA_DIGIT: StringBoolFunction = |lhs: String| query::is_alphadigit(lhs.as_str());
pub const FN_IS_BLANK: StringBoolFunction = |lhs: String| query::is_blank(lhs.as_str());
pub const FN_IS_CAMEL_CASE: StringBoolFunction = |lhs: String| query::is_camel_case(lhs.as_str());
pub const FN_IS_CAPITALIZE: StringBoolFunction = |lhs: String| query::is_capitalize(lhs.as_str());
pub const FN_IS_DECAPITALIZE: StringBoolFunction = |lhs: String| query::is_decapitalize(lhs.as_str());
pub const FN_IS_DIGIT: StringBoolFunction = |lhs: String| query::is_digit(lhs.as_str());
pub const FN_IS_EMPTY: StringBoolFunction = |lhs: String| query::is_empty(lhs.as_str());
pub const FN_IS_KEBAB_CASE: StringBoolFunction = |lhs: String| query::is_kebab_case(lhs.as_str());
pub const FN_IS_LOWER_FIRST: StringBoolFunction = |lhs: String| query::is_lower_first(lhs.as_str());
pub const FN_IS_LOWERCASE: StringBoolFunction = |lhs: String| query::is_lowercase(lhs.as_str());
pub const FN_IS_NUMERIC: StringBoolFunction = |lhs: String| query::is_numeric(lhs.as_str());
pub const FN_IS_PASCAL_CASE: StringBoolFunction = |lhs: String| query::is_pascal_case(lhs.as_str());
pub const FN_IS_SHOUTY_KEBAB_CASE: StringBoolFunction = |lhs: String| query::is_shouty_kebab_case(lhs.as_str());
pub const FN_IS_SHOUTY_SNAKE_CASE: StringBoolFunction = |lhs: String| query::is_shouty_snake_case(lhs.as_str());
pub const FN_IS_SNAKE_CASE: StringBoolFunction = |lhs: String| query::is_snake_case(lhs.as_str());
pub const FN_IS_TITLE_CASE: StringBoolFunction = |lhs: String| query::is_title(lhs.as_str());
pub const FN_IS_TRAIN_CASE: StringBoolFunction = |lhs: String| query::is_train_case(lhs.as_str());
pub const FN_IS_UPPER_FIRST: StringBoolFunction = |lhs: String| query::is_upper_first(lhs.as_str());
pub const FN_IS_UPPERCASE: StringBoolFunction = |lhs: String| query::is_uppercase(lhs.as_str());

behaviour_functions!(
    STRING_BOOL_OPERATIONS,
    StringBoolFunction,
    NAMESPACE_STRING,
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
    ("is_uppercase", FN_IS_UPPERCASE)
);
