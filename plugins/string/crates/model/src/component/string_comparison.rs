use crate::NAMESPACE_STRING;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultBoolean;
use inexor_rgf_reactive_api::entity_model;

properties!(StringComparisonProperties, (LHS, "lhs", ""), (RHS, "rhs", ""));

component_ty!(COMPONENT_STRING_COMPARISON, NAMESPACE_STRING, COMPONENT_NAME_STRING_COMPARISON, "string_comparison");

entity_model!(
    StringComparison,
    set lhs string,
    set rhs string
);
impl ResultBoolean for StringComparison {}
