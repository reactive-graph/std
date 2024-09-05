use crate::NAMESPACE_STRING;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_model_result::ResultString;
use reactive_graph_reactive_model_api::entity_model;

properties!(StringOperationProperties, (LHS, "lhs", ""));

component_ty!(COMPONENT_STRING_OPERATION, NAMESPACE_STRING, COMPONENT_NAME_STRING_OPERATION, "string_operation");

entity_model!(
    StringOperation,
    set lhs string
);
impl ResultString for StringOperation {}
