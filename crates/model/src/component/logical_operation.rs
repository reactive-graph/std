use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::NAMESPACE_LOGICAL;

properties!(LogicalOperationProperties, (LHS, "lhs", false), (RESULT, "result", false));

component_ty!(COMPONENT_LOGICAL_OPERATION, NAMESPACE_LOGICAL, COMPONENT_NAME_LOGICAL_OPERATION, "logical_operation");

entity_model!(LogicalOperation, get result bool, set lhs bool);
