use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::NAMESPACE_COMPARISON;

properties!(ComparisonGateProperties, (LHS, "lhs", false), (RHS, "rhs", false), (RESULT, "result", false));

component_ty!(COMPONENT_COMPARISON_GATE, NAMESPACE_COMPARISON, COMPONENT_NAME_COMPARISON_GATE, "comparison_gate");

entity_model!(ComparisonGate, get result bool, set lhs value, set rhs value);
