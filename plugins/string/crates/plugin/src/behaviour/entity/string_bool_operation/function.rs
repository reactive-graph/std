use inexor_rgf_behaviour_model_impl::entity::EntityBehaviourFactoryCreator;
use inexor_rgf_behaviour_model_impl::entity::EntityBehaviourFunctions;
use inexor_rgf_behaviour_model_impl::entity::EntityBehaviourFunctionsStorage;
use std::sync::Arc;
use std::sync::LazyLock;
use voca_rs::query;

use crate::behaviour::entity::string_bool_operation::StringBoolOperationFactory;
use inexor_rgf_model_string::NAMESPACE_STRING;

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

const FACTORY_CREATOR: EntityBehaviourFactoryCreator<StringBoolFunction> = |ty, f| Arc::new(StringBoolOperationFactory::new(ty.clone(), f));

pub static STRING_BOOL_OPERATIONS: EntityBehaviourFunctionsStorage<StringBoolFunction> = LazyLock::new(|| {
    EntityBehaviourFunctions::<StringBoolFunction>::with_namespace(NAMESPACE_STRING, FACTORY_CREATOR)
        .behaviour("is_alpha", FN_IS_ALPHA)
        .behaviour("is_alpha_digit", FN_IS_ALPHA_DIGIT)
        .behaviour("is_blank", FN_IS_BLANK)
        .behaviour("is_camel_case", FN_IS_CAMEL_CASE)
        .behaviour("is_capitalize", FN_IS_CAPITALIZE)
        .behaviour("is_decapitalize", FN_IS_DECAPITALIZE)
        .behaviour("is_digit", FN_IS_DIGIT)
        .behaviour("is_empty", FN_IS_EMPTY)
        .behaviour("is_kebab_case", FN_IS_KEBAB_CASE)
        .behaviour("is_lower_first", FN_IS_LOWER_FIRST)
        .behaviour("is_lowercase", FN_IS_LOWERCASE)
        .behaviour("is_numeric", FN_IS_NUMERIC)
        .behaviour("is_pascal_case", FN_IS_PASCAL_CASE)
        .behaviour("is_shouty_kebab_case", FN_IS_SHOUTY_KEBAB_CASE)
        .behaviour("is_shouty_snake_case", FN_IS_SHOUTY_SNAKE_CASE)
        .behaviour("is_snake_case", FN_IS_SNAKE_CASE)
        .behaviour("is_title_case", FN_IS_TITLE_CASE)
        .behaviour("is_train_case", FN_IS_TRAIN_CASE)
        .behaviour("is_upper_first", FN_IS_UPPER_FIRST)
        .behaviour("is_uppercase", FN_IS_UPPERCASE)
        .get()
});
