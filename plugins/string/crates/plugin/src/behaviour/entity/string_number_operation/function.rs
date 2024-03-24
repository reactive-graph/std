use std::sync::Arc;
use std::sync::LazyLock;

use inexor_rgf_behaviour_model_impl::entity::EntityBehaviourFactoryCreator;
use inexor_rgf_behaviour_model_impl::entity::EntityBehaviourFunctions;
use inexor_rgf_behaviour_model_impl::entity::EntityBehaviourFunctionsStorage;
use voca_rs::count;

use crate::behaviour::entity::string_number_operation::StringNumberOperationFactory;
use inexor_rgf_model_string::NAMESPACE_STRING;

pub type StringNumberFunction = fn(String) -> usize;

pub const FN_STRING_LENGTH: StringNumberFunction = |lhs: String| lhs.len();
pub const FN_CHAR_COUNT: StringNumberFunction = |lhs: String| count::count(lhs.as_str());
pub const FN_CHAR_COUNT_GRAPHEMES: StringNumberFunction = |lhs: String| count::count_graphemes(lhs.as_str());

const FACTORY_CREATOR: EntityBehaviourFactoryCreator<StringNumberFunction> = |ty, f| Arc::new(StringNumberOperationFactory::new(ty.clone(), f));

pub static STRING_NUMBER_OPERATIONS: EntityBehaviourFunctionsStorage<StringNumberFunction> = LazyLock::new(|| {
    EntityBehaviourFunctions::<StringNumberFunction>::with_namespace(NAMESPACE_STRING, FACTORY_CREATOR)
        .behaviour("string_length", FN_STRING_LENGTH)
        .behaviour("char_count", FN_CHAR_COUNT)
        .behaviour("char_count_graphemes", FN_CHAR_COUNT_GRAPHEMES)
        .get()
});
