use crate::model::component_type;
use crate::model::properties;
use crate::NAMESPACE_LOGICAL;

properties!(ConditionProperties, (CONDITION, "condition", false), (RESULT, "result", false));

component_type!(COMPONENT_CONDITION, NAMESPACE_LOGICAL, COMPONENT_NAME_CONDITION, "condition");
