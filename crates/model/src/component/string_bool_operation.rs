use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::NAMESPACE_STRING;

properties!(StringBoolOperationProperties, (LHS, "lhs", ""), (RESULT, "result", false));

component_ty!(
    COMPONENT_STRING_BOOL_OPERATION,
    NAMESPACE_STRING,
    COMPONENT_NAME_STRING_BOOL_OPERATION,
    "string_bool_operation"
);

entity_model!(
    StringBoolOperation,
    get result bool,
    set lhs string
);
