use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_TRIGGER;

properties!(ActionProperties, (TRIGGER, "trigger", false));

component_ty!(COMPONENT_ACTION, NAMESPACE_TRIGGER, COMPONENT_NAME_ACTION, "action");

component_model!(Action, trigger);
