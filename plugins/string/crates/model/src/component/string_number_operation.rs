use crate::NAMESPACE_STRING;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultNumberU64;
use inexor_rgf_reactive_model_api::entity_model;

properties!(StringNumberOperationProperties, (LHS, "lhs", ""));

component_ty!(
    COMPONENT_STRING_NUMBER_OPERATION,
    NAMESPACE_STRING,
    COMPONENT_NAME_STRING_NUMBER_OPERATION,
    "string_number_operation"
);

entity_model!(
    StringNumberOperation,
    set lhs string
);
impl ResultNumberU64 for StringNumberOperation {}
