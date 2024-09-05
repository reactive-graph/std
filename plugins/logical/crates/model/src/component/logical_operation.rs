use crate::NAMESPACE_LOGICAL;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_model_result::ResultBoolean;
use reactive_graph_reactive_model_api::entity_model;

properties!(LogicalOperationProperties, (LHS, "lhs", false));

component_ty!(COMPONENT_LOGICAL_OPERATION, NAMESPACE_LOGICAL, COMPONENT_NAME_LOGICAL_OPERATION, "logical_operation");

entity_model!(
    LogicalOperation,
    set lhs bool
);
impl ResultBoolean for LogicalOperation {}
