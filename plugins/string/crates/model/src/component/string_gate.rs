use crate::NAMESPACE_STRING;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_std_result_model::ResultString;

properties!(StringGateProperties, (LHS, "lhs", ""), (RHS, "rhs", ""));

component_ty!(COMPONENT_STRING_GATE, NAMESPACE_STRING, COMPONENT_NAME_STRING_GATE, "string_gate");

entity_model!(
    StringGate,
    set lhs string,
    set rhs string
);
impl ResultString for StringGate {}
