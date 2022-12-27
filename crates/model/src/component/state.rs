use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::NAMESPACE_STATE;

properties!(StateProperties, (STATE, "state", 0), (SET_STATE, "set_state", 0));

component_ty!(COMPONENT_STATE, NAMESPACE_STATE, COMPONENT_NAME_STATE, "state");

entity_model!(State, get value value, get state value, set set_state value);
entity_model!(StateBool, get value bool, get state bool, set set_state bool);
entity_model!(StateF64, get value f64, get state f64, set set_state f64);
entity_model!(StateString, get value string, get state string, set set_state string);
entity_model!(StateArray, get value array, get state array, set set_state array);
entity_model!(StateObject, get value object, get state object, set set_state object);
