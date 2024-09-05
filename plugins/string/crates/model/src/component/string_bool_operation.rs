use crate::NAMESPACE_STRING;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_model_result::ResultBoolean;
use reactive_graph_reactive_model_api::entity_model;

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
