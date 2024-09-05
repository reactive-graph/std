use std::sync::Arc;
use std::sync::LazyLock;

use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFactoryCreator;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctions;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctionsStorage;
use voca_rs::case;
use voca_rs::escape;
use voca_rs::manipulate;
use voca_rs::strip;

use reactive_graph_model_string::NAMESPACE_STRING;

use crate::behaviour::entity::string_operation::StringOperationFactory;

pub type StringOperationFunction = fn(String) -> String;

pub const FN_CAMEL_CASE: StringOperationFunction = |lhs: String| case::camel_case(lhs.as_str());
pub const FN_CAPITALIZE: StringOperationFunction = |lhs: String| case::capitalize(lhs.as_str(), true);
pub const FN_DECAPITALIZE: StringOperationFunction = |lhs: String| case::decapitalize(lhs.as_str(), true);
pub const FN_ESCAPE_HTML: StringOperationFunction = |lhs: String| escape::escape_html(lhs.as_str());
pub const FN_ESCAPE_REGEXP: StringOperationFunction = |lhs: String| escape::escape_regexp(lhs.as_str());
pub const FN_KEBAB_CASE: StringOperationFunction = |lhs: String| case::kebab_case(lhs.as_str());
pub const FN_LOWER_FIRST: StringOperationFunction = |lhs: String| case::lower_first(lhs.as_str());
pub const FN_LOWERCASE: StringOperationFunction = |lhs: String| lhs.to_lowercase();
pub const FN_PASCAL_CASE: StringOperationFunction = |lhs: String| case::pascal_case(lhs.as_str());
pub const FN_REVERSE: StringOperationFunction = |lhs: String| manipulate::reverse(lhs.as_str());
pub const FN_SHOUTY_KEBAB_CASE: StringOperationFunction = |lhs: String| case::shouty_kebab_case(lhs.as_str());
pub const FN_SHOUTY_SNAKE_CASE: StringOperationFunction = |lhs: String| case::shouty_snake_case(lhs.as_str());
pub const FN_SNAKE_CASE: StringOperationFunction = |lhs: String| case::snake_case(lhs.as_str());
pub const FN_STRIP_HTML_TAGS: StringOperationFunction = |lhs: String| strip::strip_tags(lhs.as_str());
pub const FN_SWAP_CASE: StringOperationFunction = |lhs: String| case::swap_case(lhs.as_str());
pub const FN_TITLE_CASE: StringOperationFunction = |lhs: String| case::title_case(lhs.as_str());
pub const FN_TRAIN_CASE: StringOperationFunction = |lhs: String| case::train_case(lhs.as_str());
pub const FN_TRIM: StringOperationFunction = |lhs: String| String::from(lhs.trim());
pub const FN_TRIM_START: StringOperationFunction = |lhs: String| String::from(lhs.trim_start());
pub const FN_TRIM_END: StringOperationFunction = |lhs: String| String::from(lhs.trim_end());
pub const FN_UNESCAPE_HTML: StringOperationFunction = |lhs: String| escape::unescape_html(lhs.as_str());
pub const FN_UPPER_FIRST: StringOperationFunction = |lhs: String| case::upper_first(lhs.as_str());
pub const FN_UPPERCASE: StringOperationFunction = |lhs: String| lhs.to_uppercase();

const FACTORY_CREATOR: EntityBehaviourFactoryCreator<StringOperationFunction> = |ty, f| Arc::new(StringOperationFactory::new(ty.clone(), f));

pub static STRING_OPERATIONS: EntityBehaviourFunctionsStorage<StringOperationFunction> = LazyLock::new(|| {
    EntityBehaviourFunctions::<StringOperationFunction>::with_namespace(NAMESPACE_STRING, FACTORY_CREATOR)
        .behaviour("camel_case", FN_CAMEL_CASE)
        .behaviour("capitalize", FN_CAPITALIZE)
        .behaviour("decapitalize", FN_DECAPITALIZE)
        .behaviour("escape_html", FN_ESCAPE_HTML)
        .behaviour("escape_regexp", FN_ESCAPE_REGEXP)
        .behaviour("kebab_case", FN_KEBAB_CASE)
        .behaviour("lower_first", FN_LOWER_FIRST)
        .behaviour("lowercase", FN_LOWERCASE)
        .behaviour("pascal_case", FN_PASCAL_CASE)
        .behaviour("reverse", FN_REVERSE)
        .behaviour("shouty_kebab_case", FN_SHOUTY_KEBAB_CASE)
        .behaviour("shouty_snake_case", FN_SHOUTY_SNAKE_CASE)
        .behaviour("snake_case", FN_SNAKE_CASE)
        .behaviour("strip_html_tags", FN_STRIP_HTML_TAGS)
        .behaviour("swap_case", FN_SWAP_CASE)
        .behaviour("title_case", FN_TITLE_CASE)
        .behaviour("train_case", FN_TRAIN_CASE)
        .behaviour("trim", FN_TRIM)
        .behaviour("trim_start", FN_TRIM_START)
        .behaviour("trim_end", FN_TRIM_END)
        .behaviour("unescape_html", FN_UNESCAPE_HTML)
        .behaviour("upper_first", FN_UPPER_FIRST)
        .behaviour("uppercase", FN_UPPERCASE)
        .get()
});
