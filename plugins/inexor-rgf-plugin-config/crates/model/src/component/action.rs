// TODO: remove this file and use model_logical::ActionProperties

use crate::model::component_ty;
use crate::model::properties;

pub const NAMESPACE_LOGICAL: &str = "logical";

properties!(ActionProperties, (TRIGGER, "trigger", false), (RESULT, "result", false));

component_ty!(COMPONENT_ACTION, NAMESPACE_LOGICAL, COMPONENT_NAME_ACTION, "action");
