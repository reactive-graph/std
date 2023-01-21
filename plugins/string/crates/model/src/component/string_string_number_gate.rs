use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model_result::ResultNumberU64;
use crate::NAMESPACE_STRING;

properties!(StringStringNumberGateProperties, (LHS, "lhs", ""), (RHS, "rhs", ""));

component_ty!(
    COMPONENT_STRING_STRING_NUMBER_GATE,
    NAMESPACE_STRING,
    COMPONENT_NAME_STRING_STRING_NUMBER_GATE,
    "string_string_number_gate"
);

entity_model!(
    StringStringNumberGate,
    set lhs string,
    set rhs string
);
impl ResultNumberU64 for StringStringNumberGate {}
