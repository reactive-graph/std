use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model_result::ResultBoolean;
use crate::NAMESPACE_STRING;

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
