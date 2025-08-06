use crate::NAMESPACE_OBJECT;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_std_result_model::ResultAny;

properties!(ObjectGetPropertyProperties, (OBJECT, "object", {}), (PROPERTY_NAME, "property_name", ""));

entity_ty!(ENTITY_TYPE_OBJECT_GET_PROPERTY, NAMESPACE_OBJECT, ENTITY_TYPE_NAME_OBJECT_GET_PROPERTY, "get_property");
behaviour_ty!(BEHAVIOUR_OBJECT_GET_PROPERTY, NAMESPACE_OBJECT, BEHAVIOUR_NAME_OBJECT_GET_PROPERTY, "get_property");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_OBJECT_GET_PROPERTY, ENTITY_TYPE_OBJECT_GET_PROPERTY, BEHAVIOUR_OBJECT_GET_PROPERTY);

entity_model!(
    ObjectGetProperty,
    set property_name string,
    set object object
);
impl ResultAny for ObjectGetProperty {}
