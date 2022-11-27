use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_VALUE;

properties!(ValueProperties, (VALUE, "value", ""));

component_ty!(COMPONENT_VALUE, NAMESPACE_VALUE, COMPONENT_NAME_VALUE, "value");
