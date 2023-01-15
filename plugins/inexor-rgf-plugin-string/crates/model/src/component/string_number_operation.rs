use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::NAMESPACE_STRING;

properties!(StringNumberOperationProperties, (LHS, "lhs", ""), (RESULT, "result", 0));

component_ty!(
    COMPONENT_STRING_NUMBER_OPERATION,
    NAMESPACE_STRING,
    COMPONENT_NAME_STRING_NUMBER_OPERATION,
    "string_number_operation"
);

entity_model!(
    StringNumberOperation,
    get result u64,
    set lhs string
);
