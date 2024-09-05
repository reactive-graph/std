use crate::behaviour::entity::string_comparison::StringComparisonFactory;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFactoryCreator;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctions;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctionsStorage;
use reactive_graph_model_string::NAMESPACE_STRING;
use std::sync::Arc;
use std::sync::LazyLock;

pub type StringComparisonFunction = fn(String, String) -> bool;

pub const FN_CONTAINS: StringComparisonFunction = |lhs, rhs| lhs.contains(rhs.as_str());
pub const FN_ENDS_WITH: StringComparisonFunction = |lhs, rhs| lhs.ends_with(rhs.as_str());
pub const FN_STARTS_WITH: StringComparisonFunction = |lhs, rhs| lhs.starts_with(rhs.as_str());

const FACTORY_CREATOR: EntityBehaviourFactoryCreator<StringComparisonFunction> = |ty, f| Arc::new(StringComparisonFactory::new(ty.clone(), f));

pub static STRING_COMPARISONS: EntityBehaviourFunctionsStorage<StringComparisonFunction> = LazyLock::new(|| {
    EntityBehaviourFunctions::<StringComparisonFunction>::with_namespace(NAMESPACE_STRING, FACTORY_CREATOR)
        .behaviour("contains", FN_CONTAINS)
        .behaviour("ends_with", FN_ENDS_WITH)
        .behaviour("starts_with", FN_STARTS_WITH)
        .get()
});
