use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::NAMESPACE_STRING;

properties!(StringNumberOperationProperties, (LHS, "lhs", ""), (RESULT, "result", 0.0));

component_ty!(
    COMPONENT_STRING_NUMBER_OPERATION,
    NAMESPACE_STRING,
    COMPONENT_NAME_STRING_NUMBER_OPERATION,
    "string_number_operation"
);

entity_model!(
    StringNumberOperation,
    get result f64,
    set lhs string
);
