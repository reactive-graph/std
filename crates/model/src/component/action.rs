use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_LOGICAL;

properties!(ActionProperties, (TRIGGER, "trigger", false), (RESULT, "result", false));

component_ty!(COMPONENT_ACTION, NAMESPACE_LOGICAL, COMPONENT_NAME_ACTION, "action");

component_model!(
    Action,
    trigger,
    get result value,
);
