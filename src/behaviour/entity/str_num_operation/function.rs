use lazy_static::lazy_static;
use std::collections::HashMap;
use voca_rs::count;

pub type StrNumFunction = fn(String) -> usize;

pub const FN_STRING_LENGTH: StrNumFunction = |lhs: String| lhs.len();
pub const FN_CHAR_COUNT: StrNumFunction = |lhs: String| count::count(lhs.as_str());
pub const FN_CHAR_COUNT_GRAPHEMES: StrNumFunction = |lhs: String| count::count_graphemes(lhs.as_str());

lazy_static! {
    pub static ref STR_NUM_FUNCTIONS: HashMap<&'static str, StrNumFunction> = vec![
        ("string_length", FN_STRING_LENGTH),
        ("char_count", FN_CHAR_COUNT),
        ("char_count_graphemes", FN_CHAR_COUNT_GRAPHEMES)
    ]
    .into_iter()
    .collect();
}
