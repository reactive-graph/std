use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_STATE;

properties!(StateProperties, (STATE, "state", 0), (SET_STATE, "set_state", 0));

component_ty!(COMPONENT_STATE, NAMESPACE_STATE, COMPONENT_NAME_STATE, "state");

component_model!(SetState, set set_state value);

component_model!(SetStateBoolean, set set_state bool);

component_model!(SetStateNumber, set set_state f64);

component_model!(SetStateString, set set_state string);

component_model!(SetStateArray, set set_state array);

component_model!(SetStateObject, set set_state object);
