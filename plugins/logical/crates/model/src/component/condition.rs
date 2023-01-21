use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_LOGICAL;

properties!(ConditionProperties, (CONDITION, "condition", false));

component_ty!(COMPONENT_CONDITION, NAMESPACE_LOGICAL, COMPONENT_NAME_CONDITION, "condition");

component_model!(
    Condition,
    set condition bool,
);
