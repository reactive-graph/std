use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_STATE;

properties!(StateProperties, (STATE, "state", 0), (SET_STATE, "set_state", 0));

component_ty!(COMPONENT_STATE, NAMESPACE_STATE, COMPONENT_NAME_STATE, "state");
