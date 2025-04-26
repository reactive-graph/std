use crate::NAMESPACE_OBJECT;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_std_result_model::ResultAny;

properties!(
    ObjectSetPropertyProperties,
    (OBJECT, "object", {}),
    (PROPERTY_NAME, "property_name", ""),
    (VALUE, "value", "")
);

entity_ty!(ENTITY_TYPE_OBJECT_SET_PROPERTY, NAMESPACE_OBJECT, ENTITY_TYPE_NAME_OBJECT_SET_PROPERTY, "object_set_property");
behaviour_ty!(BEHAVIOUR_OBJECT_SET_PROPERTY, NAMESPACE_OBJECT, BEHAVIOUR_NAME_OBJECT_SET_PROPERTY, "object_set_property");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_OBJECT_SET_PROPERTY, ENTITY_TYPE_OBJECT_SET_PROPERTY, BEHAVIOUR_OBJECT_SET_PROPERTY);

entity_model!(
    ObjectSetProperty,
    set property_name string,
    set value value,
    set object object
);
impl ResultAny for ObjectSetProperty {}
