use std::sync::Arc;
use std::sync::LazyLock;

use inexor_rgf_behaviour::entity::EntityBehaviourFactoryCreator;
use inexor_rgf_behaviour::entity::EntityBehaviourFunctions;
use inexor_rgf_behaviour::entity::EntityBehaviourFunctionsStorage;

use crate::behaviour::entity::gate::LogicalGateFactory;
use inexor_rgf_model_logical::NAMESPACE_LOGICAL;

pub type LogicalGateFunction = fn(bool, bool) -> bool;

pub const FN_AND: LogicalGateFunction = |lhs, rhs| lhs && rhs;
pub const FN_NAND: LogicalGateFunction = |lhs, rhs| !(lhs && rhs);
pub const FN_NOR: LogicalGateFunction = |lhs, rhs| !(lhs || rhs);
pub const FN_OR: LogicalGateFunction = |lhs, rhs| lhs || rhs;
pub const FN_XOR: LogicalGateFunction = |lhs, rhs| lhs ^ rhs;
pub const FN_XNOR: LogicalGateFunction = |lhs, rhs| !(lhs ^ rhs);

const FACTORY_CREATOR: EntityBehaviourFactoryCreator<LogicalGateFunction> = |ty, f| Arc::new(LogicalGateFactory::new(ty.clone(), f));

pub static LOGICAL_GATES: EntityBehaviourFunctionsStorage<LogicalGateFunction> = LazyLock::new(|| {
    EntityBehaviourFunctions::<LogicalGateFunction>::with_namespace(NAMESPACE_LOGICAL, FACTORY_CREATOR)
        .behaviour("and", FN_AND)
        .behaviour("nand", FN_NAND)
        .behaviour("nor", FN_NOR)
        .behaviour("or", FN_OR)
        .behaviour("xor", FN_XOR)
        .behaviour("xnor", FN_XNOR)
        .get()
});
