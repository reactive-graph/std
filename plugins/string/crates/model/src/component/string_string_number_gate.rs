use crate::NAMESPACE_STRING;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultNumberU64;
use inexor_rgf_reactive_model_api::entity_model;

properties!(StringStringNumberGateProperties, (LHS, "lhs", ""), (RHS, "rhs", ""));

component_ty!(
    COMPONENT_STRING_STRING_NUMBER_GATE,
    NAMESPACE_STRING,
    COMPONENT_NAME_STRING_STRING_NUMBER_GATE,
    "string_string_number_gate"
);

entity_model!(
    StringStringNumberGate,
    set lhs string,
    set rhs string
);
impl ResultNumberU64 for StringStringNumberGate {}
