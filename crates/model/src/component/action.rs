use crate::model::component_type;
use crate::model::properties;
use crate::NAMESPACE_LOGICAL;

properties!(ActionProperties, (TRIGGER, "trigger", false), (RESULT, "result", false));

component_type!(COMPONENT_ACTION, NAMESPACE_LOGICAL, COMPONENT_NAME_ACTION, "action");
