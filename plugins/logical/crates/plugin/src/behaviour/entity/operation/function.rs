use std::sync::Arc;
use std::sync::LazyLock;

use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFactoryCreator;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctions;
use reactive_graph_behaviour_model_impl::entity::EntityBehaviourFunctionsStorage;

use crate::behaviour::entity::operation::LogicalOperationFactory;
use reactive_graph_std_logical_model::NAMESPACE_LOGICAL;

pub type LogicalOperationFunction = fn(bool) -> bool;

pub const FN_NOT: LogicalOperationFunction = |lhs| !lhs;

const FACTORY_CREATOR: EntityBehaviourFactoryCreator<LogicalOperationFunction> = |ty, f| Arc::new(LogicalOperationFactory::new(ty.clone(), f));

pub static LOGICAL_OPERATIONS: EntityBehaviourFunctionsStorage<LogicalOperationFunction> = LazyLock::new(|| {
    EntityBehaviourFunctions::<LogicalOperationFunction>::with_namespace(NAMESPACE_LOGICAL, FACTORY_CREATOR)
        .behaviour("not", FN_NOT)
        .get()
});
