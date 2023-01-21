use voca_rs::count;

use crate::model_string::NAMESPACE_STRING;
use crate::reactive::behaviour_functions;

pub type StringStringNumberFunction = fn(String, String) -> usize;

pub const FN_COUNT_SUBSTRINGS: StringStringNumberFunction = |lhs, rhs| count::count_substrings(lhs.as_str(), rhs.as_str());
pub const FN_COUNT_UNIQUE_WORDS: StringStringNumberFunction = |lhs, rhs| count::count_unique_words(lhs.as_str(), rhs.as_str());
pub const FN_COUNT_WORDS: StringStringNumberFunction = |lhs, rhs| count::count_words(lhs.as_str(), rhs.as_str());

behaviour_functions!(
    STRING_STRING_NUMBER_GATES,
    StringStringNumberFunction,
    NAMESPACE_STRING,
    ("count_substrings", FN_COUNT_SUBSTRINGS),
    ("count_unique_words", FN_COUNT_UNIQUE_WORDS),
    ("count_words", FN_COUNT_WORDS)
);
