use std::sync::Arc;
use std::sync::LazyLock;

use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFactoryCreator;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctions;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctionsStorage;
use voca_rs::count;

use crate::behaviour::entity::string_number_operation::StringNumberOperationFactory;
use reactive_graph_std_string_model::NAMESPACE_STRING;

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
