use crate::NAMESPACE_LOGICAL;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultBoolean;
use inexor_rgf_reactive_model_api::entity_model;

properties!(LogicalOperationProperties, (LHS, "lhs", false));

component_ty!(COMPONENT_LOGICAL_OPERATION, NAMESPACE_LOGICAL, COMPONENT_NAME_LOGICAL_OPERATION, "logical_operation");

entity_model!(
    LogicalOperation,
    set lhs bool
);
impl ResultBoolean for LogicalOperation {}
