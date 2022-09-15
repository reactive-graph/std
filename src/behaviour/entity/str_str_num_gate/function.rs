use lazy_static::lazy_static;
use std::collections::HashMap;
use voca_rs::count;

pub type StrStrNumFunction = fn(String, String) -> usize;

pub const FN_COUNT_SUBSTRINGS: StrStrNumFunction = |lhs, rhs| count::count_substrings(lhs.as_str(), rhs.as_str());
pub const FN_COUNT_UNIQUE_WORDS: StrStrNumFunction = |lhs, rhs| count::count_unique_words(lhs.as_str(), rhs.as_str());
pub const FN_COUNT_WORDS: StrStrNumFunction = |lhs, rhs| count::count_words(lhs.as_str(), rhs.as_str());

lazy_static! {
    pub static ref STR_STR_NUM_FUNCTIONS: HashMap<&'static str, StrStrNumFunction> = vec![
        ("count_substrings", FN_COUNT_SUBSTRINGS),
        ("count_unique_words", FN_COUNT_UNIQUE_WORDS),
        ("count_words", FN_COUNT_WORDS),
    ]
    .into_iter()
    .collect();
}
