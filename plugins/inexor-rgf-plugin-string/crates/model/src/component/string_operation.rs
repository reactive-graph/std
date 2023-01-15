use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::NAMESPACE_STRING;

properties!(StringOperationProperties, (LHS, "lhs", ""), (RESULT, "result", ""));

component_ty!(COMPONENT_STRING_OPERATION, NAMESPACE_STRING, COMPONENT_NAME_STRING_OPERATION, "string_operation");

entity_model!(
    StringOperation,
    get result string,
    set lhs string
);
