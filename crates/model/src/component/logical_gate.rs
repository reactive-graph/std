use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::NAMESPACE_LOGICAL;

properties!(LogicalGateProperties, (LHS, "lhs", false), (RHS, "rhs", false), (RESULT, "result", false));

component_ty!(COMPONENT_LOGICAL_GATE, NAMESPACE_LOGICAL, COMPONENT_NAME_LOGICAL_GATE, "logical_gate");

entity_model!(LogicalGate, get result bool, set lhs bool, set rhs bool);
