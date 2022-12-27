use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::NAMESPACE_STRING;

properties!(StringGateProperties, (LHS, "lhs", ""), (RHS, "rhs", ""), (RESULT, "result", ""));

component_ty!(COMPONENT_STRING_GATE, NAMESPACE_STRING, COMPONENT_NAME_STRING_GATE, "string_gate");

entity_model!(
    StringGateOperation,
    get result string,
    set lhs string,
    set rhs string
);
