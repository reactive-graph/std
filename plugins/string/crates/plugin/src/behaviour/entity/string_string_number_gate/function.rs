use std::sync::Arc;
use std::sync::LazyLock;

use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFactoryCreator;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctions;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctionsStorage;
use voca_rs::count;

use reactive_graph_std_string_model::NAMESPACE_STRING;

use crate::behaviour::entity::string_string_number_gate::StringStringNumberGateFactory;

pub type StringStringNumberFunction = fn(String, String) -> usize;

pub const FN_COUNT_SUBSTRINGS: StringStringNumberFunction = |lhs, rhs| count::count_substrings(lhs.as_str(), rhs.as_str());
pub const FN_COUNT_UNIQUE_WORDS: StringStringNumberFunction = |lhs, rhs| count::count_unique_words(lhs.as_str(), rhs.as_str());
pub const FN_COUNT_WORDS: StringStringNumberFunction = |lhs, rhs| count::count_words(lhs.as_str(), rhs.as_str());

const FACTORY_CREATOR: EntityBehaviourFactoryCreator<StringStringNumberFunction> = |ty, f| Arc::new(StringStringNumberGateFactory::new(ty.clone(), f));

pub static STRING_STRING_NUMBER_GATES: EntityBehaviourFunctionsStorage<StringStringNumberFunction> = LazyLock::new(|| {
    EntityBehaviourFunctions::<StringStringNumberFunction>::with_namespace(NAMESPACE_STRING, FACTORY_CREATOR)
        .behaviour("count_substrings", FN_COUNT_SUBSTRINGS)
        .behaviour("count_unique_words", FN_COUNT_UNIQUE_WORDS)
        .behaviour("count_words", FN_COUNT_WORDS)
        .get()
});
