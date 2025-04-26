use crate::NAMESPACE_LOGICAL;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_std_result_model::ResultBoolean;

properties!(LogicalGateProperties, (LHS, "lhs", false), (RHS, "rhs", false));

component_ty!(COMPONENT_LOGICAL_GATE, NAMESPACE_LOGICAL, COMPONENT_NAME_LOGICAL_GATE, "logical_gate");

entity_model!(
    LogicalGate,
    set lhs bool,
    set rhs bool
);
impl ResultBoolean for LogicalGate {}
