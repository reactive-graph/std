use crate::NAMESPACE_LOGICAL;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(ConditionProperties, (CONDITION, "condition", false));

component_ty!(COMPONENT_CONDITION, NAMESPACE_LOGICAL, COMPONENT_NAME_CONDITION, "condition");

component_model!(
    Condition,
    set condition bool,
);
