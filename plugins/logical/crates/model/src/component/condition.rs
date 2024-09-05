use crate::NAMESPACE_LOGICAL;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(ConditionProperties, (CONDITION, "condition", false));

component_ty!(COMPONENT_CONDITION, NAMESPACE_LOGICAL, COMPONENT_NAME_CONDITION, "condition");

component_model!(
    Condition,
    set condition bool,
);
