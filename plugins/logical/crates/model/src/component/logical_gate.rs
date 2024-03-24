use crate::NAMESPACE_LOGICAL;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultBoolean;
use inexor_rgf_reactive_model_api::entity_model;

properties!(LogicalGateProperties, (LHS, "lhs", false), (RHS, "rhs", false));

component_ty!(COMPONENT_LOGICAL_GATE, NAMESPACE_LOGICAL, COMPONENT_NAME_LOGICAL_GATE, "logical_gate");

entity_model!(
    LogicalGate,
    set lhs bool,
    set rhs bool
);
impl ResultBoolean for LogicalGate {}
