use voca_rs::count;

use crate::model_string::NAMESPACE_STRING;
use crate::reactive::behaviour_functions;

pub type StringNumberFunction = fn(String) -> usize;

pub const FN_STRING_LENGTH: StringNumberFunction = |lhs: String| lhs.len();
pub const FN_CHAR_COUNT: StringNumberFunction = |lhs: String| count::count(lhs.as_str());
pub const FN_CHAR_COUNT_GRAPHEMES: StringNumberFunction = |lhs: String| count::count_graphemes(lhs.as_str());

behaviour_functions!(
    STRING_NUMBER_OPERATION,
    StringNumberFunction,
    NAMESPACE_STRING,
    ("string_length", FN_STRING_LENGTH),
    ("char_count", FN_CHAR_COUNT),
    ("char_count_graphemes", FN_CHAR_COUNT_GRAPHEMES)
);
