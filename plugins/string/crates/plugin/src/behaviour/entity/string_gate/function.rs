use std::sync::Arc;
use std::sync::LazyLock;

use inexor_rgf_behaviour::entity::EntityBehaviourFactoryCreator;
use inexor_rgf_behaviour::entity::EntityBehaviourFunctions;
use inexor_rgf_behaviour::entity::EntityBehaviourFunctionsStorage;
use voca_rs::chop;

use crate::behaviour::entity::string_gate::StringGateFactory;
use inexor_rgf_model_string::NAMESPACE_STRING;

pub type StringGateFunction = fn(String, String) -> String;

pub const FN_CHOP_AFTER: StringGateFunction = |lhs, rhs| chop::after(lhs.as_str(), rhs.as_str());
pub const FN_CHOP_AFTER_LAST: StringGateFunction = |lhs, rhs| chop::after_last(lhs.as_str(), rhs.as_str());
pub const FN_CONCAT: StringGateFunction = |lhs, rhs| format!("{}{}", lhs, rhs);
pub const FN_BEFORE: StringGateFunction = |lhs, rhs| chop::before(lhs.as_str(), rhs.as_str());
pub const FN_BEFORE_LAST: StringGateFunction = |lhs, rhs| chop::before_last(lhs.as_str(), rhs.as_str());
pub const FN_REMOVE_PREFIX: StringGateFunction = |lhs, rhs| chop::removeprefix(lhs.as_str(), rhs.as_str());
pub const FN_REMOVE_SUFFIX: StringGateFunction = |lhs, rhs| chop::removesuffix(lhs.as_str(), rhs.as_str());

const FACTORY_CREATOR: EntityBehaviourFactoryCreator<StringGateFunction> = |ty, f| Arc::new(StringGateFactory::new(ty.clone(), f));

pub static STRING_GATES: EntityBehaviourFunctionsStorage<StringGateFunction> = LazyLock::new(|| {
    EntityBehaviourFunctions::<StringGateFunction>::with_namespace(NAMESPACE_STRING, FACTORY_CREATOR)
        .behaviour("chop_after", FN_CHOP_AFTER)
        .behaviour("chop_after_last", FN_CHOP_AFTER_LAST)
        .behaviour("concat", FN_CONCAT)
        .behaviour("chop_before", FN_BEFORE)
        .behaviour("chop_before_last", FN_BEFORE_LAST)
        .behaviour("chop_remove_prefix", FN_REMOVE_PREFIX)
        .behaviour("chop_remove_suffix", FN_REMOVE_SUFFIX)
        .get()
});
