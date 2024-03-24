use crate::NAMESPACE_STRING;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultString;
use inexor_rgf_reactive_model_api::entity_model;

properties!(StringOperationProperties, (LHS, "lhs", ""));

component_ty!(COMPONENT_STRING_OPERATION, NAMESPACE_STRING, COMPONENT_NAME_STRING_OPERATION, "string_operation");

entity_model!(
    StringOperation,
    set lhs string
);
impl ResultString for StringOperation {}
