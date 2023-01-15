use crate::model::entity_model;
use crate::model::entity_ty;
use crate::DataValue;
use crate::DataValueArray;
use crate::DataValueBoolean;
use crate::DataValueNumber;
use crate::DataValueObject;
use crate::DataValueString;
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
