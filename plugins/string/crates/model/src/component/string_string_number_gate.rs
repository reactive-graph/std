use crate::NAMESPACE_STRING;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_std_result_model::ResultNumberU64;

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
