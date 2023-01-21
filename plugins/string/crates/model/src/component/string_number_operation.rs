use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model_result::ResultNumberU64;
use crate::NAMESPACE_STRING;

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
