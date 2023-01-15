use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_JSON;

properties!(
    ObjectGetPropertyProperties,
    (OBJECT, "object", {}),
    (PROPERTY_NAME, "property_name", ""),
    (RESULT, "result", false)
);

entity_ty!(ENTITY_TYPE_OBJECT_GET_PROPERTY, NAMESPACE_JSON, ENTITY_TYPE_NAME_OBJECT_GET_PROPERTY, "object_get_property");
behaviour_ty!(BEHAVIOUR_OBJECT_GET_PROPERTY, NAMESPACE_JSON, BEHAVIOUR_NAME_OBJECT_GET_PROPERTY, "object_get_property");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_OBJECT_GET_PROPERTY, ENTITY_TYPE_OBJECT_GET_PROPERTY, BEHAVIOUR_OBJECT_GET_PROPERTY);

entity_model!(
    ObjectGetProperty,
    get result value,
    set property_name string,
    set object object
);
