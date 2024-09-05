use crate::NAMESPACE_STRING;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_model_result::ResultBoolean;
use reactive_graph_reactive_model_api::entity_model;

properties!(StringComparisonProperties, (LHS, "lhs", ""), (RHS, "rhs", ""));

component_ty!(COMPONENT_STRING_COMPARISON, NAMESPACE_STRING, COMPONENT_NAME_STRING_COMPARISON, "string_comparison");

entity_model!(
    StringComparison,
    set lhs string,
    set rhs string
);
impl ResultBoolean for StringComparison {}
