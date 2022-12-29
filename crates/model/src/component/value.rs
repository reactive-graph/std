use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_VALUE;

properties!(ValueProperties, (VALUE, "value", ""));

component_ty!(COMPONENT_VALUE, NAMESPACE_VALUE, COMPONENT_NAME_VALUE, "value");

component_model!(GetValue, get value value);
component_model!(DataValue, data value value);

component_model!(GetValueBoolean, get value bool);
component_model!(DataValueBoolean, data value bool);

component_model!(GetValueNumber, get value f64);
component_model!(DataValueNumber, data value f64);

component_model!(GetValueString, get value string);
component_model!(DataValueString, data value string);

component_model!(GetValueArray, get value array);
component_model!(DataValueArray, data value array);

component_model!(GetValueObject, get value object);
component_model!(DataValueObject, data value object);
