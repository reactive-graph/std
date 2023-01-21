use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model_result::ResultBoolean;
use crate::NAMESPACE_LOGICAL;

properties!(LogicalOperationProperties, (LHS, "lhs", false));

component_ty!(COMPONENT_LOGICAL_OPERATION, NAMESPACE_LOGICAL, COMPONENT_NAME_LOGICAL_OPERATION, "logical_operation");

entity_model!(
    LogicalOperation,
    set lhs bool
);
impl ResultBoolean for LogicalOperation {}
