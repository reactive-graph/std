use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model_result::ResultBoolean;
use crate::NAMESPACE_LOGICAL;

properties!(LogicalGateProperties, (LHS, "lhs", false), (RHS, "rhs", false));

component_ty!(COMPONENT_LOGICAL_GATE, NAMESPACE_LOGICAL, COMPONENT_NAME_LOGICAL_GATE, "logical_gate");

entity_model!(
    LogicalGate,
    set lhs bool,
    set rhs bool
);
impl ResultBoolean for LogicalGate {}
