use crate::NAMESPACE_STRING;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultString;
use inexor_rgf_reactive_api::entity_model;

properties!(StringGateProperties, (LHS, "lhs", ""), (RHS, "rhs", ""));

component_ty!(COMPONENT_STRING_GATE, NAMESPACE_STRING, COMPONENT_NAME_STRING_GATE, "string_gate");

entity_model!(
    StringGate,
    set lhs string,
    set rhs string
);
impl ResultString for StringGate {}
