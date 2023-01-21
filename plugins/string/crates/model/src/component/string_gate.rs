use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model_result::ResultString;
use crate::NAMESPACE_STRING;

properties!(StringGateProperties, (LHS, "lhs", ""), (RHS, "rhs", ""));

component_ty!(COMPONENT_STRING_GATE, NAMESPACE_STRING, COMPONENT_NAME_STRING_GATE, "string_gate");

entity_model!(
    StringGate,
    set lhs string,
    set rhs string
);
impl ResultString for StringGate {}
