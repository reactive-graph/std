use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model::PropertyInstanceGetter;
use crate::NAMESPACE_VALUE;

properties!(ValueProperties, (VALUE, "value", ""));

component_ty!(COMPONENT_VALUE, NAMESPACE_VALUE, COMPONENT_NAME_VALUE, "value");

entity_model!(Value, get value value);
entity_model!(ValueBool, get value bool);
entity_model!(ValueF64, get value f64);
entity_model!(ValueString, get value string);
entity_model!(ValueArray, get value array);
entity_model!(ValueObject, get value object);
