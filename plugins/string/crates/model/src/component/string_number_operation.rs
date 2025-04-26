use crate::NAMESPACE_STRING;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_std_result_model::ResultNumberU64;

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
