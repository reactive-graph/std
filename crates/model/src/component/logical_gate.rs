use crate::model::component_type;
use crate::model::properties;
use crate::NAMESPACE_LOGICAL;

properties!(LogicalGateProperties, (LHS, "lhs", false), (RHS, "rhs", false), (RESULT, "result", false));

component_type!(COMPONENT_LOGICAL_GATE, NAMESPACE_LOGICAL, COMPONENT_NAME_LOGICAL_GATE, "logical_gate");
