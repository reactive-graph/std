use crate::NAMESPACE_JSON;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;
use reactive_graph_model_result::ResultAny;
use reactive_graph_reactive_model_api::entity_model;

properties!(
    ObjectSetPropertyProperties,
    (OBJECT, "object", {}),
    (PROPERTY_NAME, "property_name", ""),
    (VALUE, "value", "")
);

entity_ty!(ENTITY_TYPE_OBJECT_SET_PROPERTY, NAMESPACE_JSON, ENTITY_TYPE_NAME_OBJECT_SET_PROPERTY, "object_set_property");
behaviour_ty!(BEHAVIOUR_OBJECT_SET_PROPERTY, NAMESPACE_JSON, BEHAVIOUR_NAME_OBJECT_SET_PROPERTY, "object_set_property");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_OBJECT_SET_PROPERTY, ENTITY_TYPE_OBJECT_SET_PROPERTY, BEHAVIOUR_OBJECT_SET_PROPERTY);

entity_model!(
    ObjectSetProperty,
    set property_name string,
    set value value,
    set object object
);
impl ResultAny for ObjectSetProperty {}
