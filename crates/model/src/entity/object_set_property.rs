use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::NAMESPACE_JSON;

properties!(
    ObjectSetPropertyProperties,
    (OBJECT, "object", {}),
    (PROPERTY_NAME, "property_name", ""),
    (VALUE, "value", ""),
    (RESULT, "result", false)
);

entity_ty!(ENTITY_TYPE_OBJECT_SET_PROPERTY, NAMESPACE_JSON, ENTITY_TYPE_NAME_OBJECT_SET_PROPERTY, "object_set_property");
behaviour_ty!(BEHAVIOUR_OBJECT_SET_PROPERTY, NAMESPACE_JSON, BEHAVIOUR_NAME_OBJECT_SET_PROPERTY, "object_set_property");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_OBJECT_SET_PROPERTY, ENTITY_TYPE_OBJECT_SET_PROPERTY, BEHAVIOUR_OBJECT_SET_PROPERTY);

entity_model!(
    ObjectSetProperty,
    get result value,
    set property_name string,
    set value value,
    set object object
);
