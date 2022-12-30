use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::NAMESPACE_STRING;

properties!(StringStringNumberGateProperties, (LHS, "lhs", ""), (RHS, "rhs", ""), (RESULT, "result", 0.0));

component_ty!(
    COMPONENT_STRING_STRING_NUMBER_GATE,
    NAMESPACE_STRING,
    COMPONENT_NAME_STRING_STRING_NUMBER_GATE,
    "string_string_number_gate"
);

entity_model!(
    StringStringNumberGate,
    get result f64,
    set lhs string,
    set rhs string
);
