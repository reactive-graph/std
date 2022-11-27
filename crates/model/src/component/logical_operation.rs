use crate::model::component_type;
use crate::model::properties;
use crate::NAMESPACE_LOGICAL;

properties!(LogicalOperationProperties, (LHS, "lhs", false), (RESULT, "result", false));

component_type!(COMPONENT_LOGICAL_OPERATION, NAMESPACE_LOGICAL, COMPONENT_NAME_LOGICAL_OPERATION, "logical_operation");
