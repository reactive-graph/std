use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model_result::ResultBoolean;
use crate::NAMESPACE_COMPARISON;

properties!(ComparisonGateProperties, (LHS, "lhs", false), (RHS, "rhs", false));

component_ty!(COMPONENT_COMPARISON_GATE, NAMESPACE_COMPARISON, COMPONENT_NAME_COMPARISON_GATE, "comparison_gate");

entity_model!(ComparisonGate, set lhs value, set rhs value);
impl ResultBoolean for ComparisonGate {}
