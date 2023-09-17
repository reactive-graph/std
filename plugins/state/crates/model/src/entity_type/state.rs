use crate::model_value::GetValue;
use crate::model_value::GetValueArray;
use crate::model_value::GetValueBoolean;
use crate::model_value::GetValueNumber;
use crate::model_value::GetValueObject;
use crate::model_value::GetValueString;
use crate::SetState;
use crate::SetStateArray;
use crate::SetStateBoolean;
use crate::SetStateNumber;
use crate::SetStateObject;
use crate::SetStateString;
use crate::NAMESPACE_STATE;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_reactive_api::entity_model;

entity_ty!(ENTITY_TYPE_STATE_BOOLEAN, NAMESPACE_STATE, ENTITY_TYPE_NAME_STATE_BOOLEAN, "state_boolean");
entity_ty!(ENTITY_TYPE_STATE_NUMBER, NAMESPACE_STATE, ENTITY_TYPE_NAME_STATE_NUMBER, "state_number");
entity_ty!(ENTITY_TYPE_STATE_STRING, NAMESPACE_STATE, ENTITY_TYPE_NAME_STATE_STRING, "state_string");
entity_ty!(ENTITY_TYPE_STATE_ARRAY, NAMESPACE_STATE, ENTITY_TYPE_NAME_STATE_ARRAY, "state_array");
entity_ty!(ENTITY_TYPE_STATE_OBJECT, NAMESPACE_STATE, ENTITY_TYPE_NAME_STATE_OBJECT, "state_object");

entity_model!(State);
impl GetValue for State {}
impl SetState for State {}

entity_model!(StateBoolean);
impl GetValueBoolean for StateBoolean {}
impl SetStateBoolean for StateBoolean {}

entity_model!(StateNumber);
impl GetValueNumber for StateNumber {}
impl SetStateNumber for StateNumber {}

entity_model!(StateString);
impl GetValueString for StateString {}
impl SetStateString for StateString {}

entity_model!(StateArray);
impl GetValueArray for StateArray {}
impl SetStateArray for StateArray {}

entity_model!(StateObject);
impl GetValueObject for StateObject {}
impl SetStateObject for StateObject {}
