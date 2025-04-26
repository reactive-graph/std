use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;

use reactive_graph_std_result_model::ResultBoolean;

use crate::NAMESPACE_COMPARISON;

properties!(ComparisonGateProperties, (LHS, "lhs", false), (RHS, "rhs", false));

component_ty!(COMPONENT_COMPARISON_GATE, NAMESPACE_COMPARISON, COMPONENT_NAME_COMPARISON_GATE, "comparison_gate");

entity_model!(ComparisonGate, set lhs value, set rhs value);
impl ResultBoolean for ComparisonGate {}
