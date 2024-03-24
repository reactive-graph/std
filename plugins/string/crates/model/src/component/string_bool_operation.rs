use crate::NAMESPACE_STRING;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultBoolean;
use inexor_rgf_reactive_model_api::entity_model;

properties!(StringBoolOperationProperties, (LHS, "lhs", ""));

component_ty!(
    COMPONENT_STRING_BOOL_OPERATION,
    NAMESPACE_STRING,
    COMPONENT_NAME_STRING_BOOL_OPERATION,
    "string_bool_operation"
);

entity_model!(
    StringBoolOperation,
    set lhs string
);
impl ResultBoolean for StringBoolOperation {}
