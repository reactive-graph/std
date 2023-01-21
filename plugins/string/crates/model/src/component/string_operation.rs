use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model_result::ResultString;
use crate::NAMESPACE_STRING;

properties!(StringOperationProperties, (LHS, "lhs", ""));

component_ty!(COMPONENT_STRING_OPERATION, NAMESPACE_STRING, COMPONENT_NAME_STRING_OPERATION, "string_operation");

entity_model!(
    StringOperation,
    set lhs string
);
impl ResultString for StringOperation {}
