use inexor_rgf_graph::entity_ty;
use inexor_rgf_reactive_api::entity_model;

use crate::component::value::DataValue;
use crate::component::value::DataValueArray;
use crate::component::value::DataValueBoolean;
use crate::component::value::DataValueNumber;
use crate::component::value::DataValueObject;
use crate::component::value::DataValueString;
use crate::NAMESPACE_VALUE;

entity_ty!(ENTITY_TYPE_VALUE_BOOLEAN, NAMESPACE_VALUE, ENTITY_TYPE_NAME_VALUE_BOOLEAN, "value_boolean");
entity_ty!(ENTITY_TYPE_VALUE_NUMBER, NAMESPACE_VALUE, ENTITY_TYPE_NAME_VALUE_NUMBER, "value_number");
entity_ty!(ENTITY_TYPE_VALUE_STRING, NAMESPACE_VALUE, ENTITY_TYPE_NAME_VALUE_STRING, "value_string");
entity_ty!(ENTITY_TYPE_VALUE_ARRAY, NAMESPACE_VALUE, ENTITY_TYPE_NAME_VALUE_ARRAY, "value_array");
entity_ty!(ENTITY_TYPE_VALUE_OBJECT, NAMESPACE_VALUE, ENTITY_TYPE_NAME_VALUE_OBJECT, "value_object");

entity_model!(Value);
impl DataValue for Value {}

entity_model!(ValueBoolean);
impl DataValueBoolean for ValueBoolean {}

entity_model!(ValueNumber);
impl DataValueNumber for ValueNumber {}

entity_model!(ValueString);
impl DataValueString for ValueString {}

entity_model!(ValueArray);
impl DataValueArray for ValueArray {}

entity_model!(ValueObject);
impl DataValueObject for ValueObject {}
