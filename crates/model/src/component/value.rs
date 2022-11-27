use crate::model::component_type;
use crate::model::properties;
use crate::NAMESPACE_VALUE;

properties!(ValueProperties, (VALUE, "value", ""));

component_type!(COMPONENT_VALUE, NAMESPACE_VALUE, COMPONENT_NAME_VALUE, "value");
