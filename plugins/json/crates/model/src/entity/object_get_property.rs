use crate::NAMESPACE_JSON;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;
use reactive_graph_model_result::ResultAny;
use reactive_graph_reactive_model_api::entity_model;

properties!(ObjectGetPropertyProperties, (OBJECT, "object", {}), (PROPERTY_NAME, "property_name", ""));

entity_ty!(ENTITY_TYPE_OBJECT_GET_PROPERTY, NAMESPACE_JSON, ENTITY_TYPE_NAME_OBJECT_GET_PROPERTY, "object_get_property");
behaviour_ty!(BEHAVIOUR_OBJECT_GET_PROPERTY, NAMESPACE_JSON, BEHAVIOUR_NAME_OBJECT_GET_PROPERTY, "object_get_property");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_OBJECT_GET_PROPERTY, ENTITY_TYPE_OBJECT_GET_PROPERTY, BEHAVIOUR_OBJECT_GET_PROPERTY);

entity_model!(
    ObjectGetProperty,
    set property_name string,
    set object object
);
impl ResultAny for ObjectGetProperty {}
