use crate::NAMESPACE_JSON;
use inexor_rgf_behaviour_model_api::behaviour_ty;
use inexor_rgf_behaviour_model_api::entity_behaviour_ty;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultAny;
use inexor_rgf_reactive_model_api::entity_model;

properties!(ObjectRemovePropertyProperties, (OBJECT, "object", {}), (PROPERTY_NAME, "property_name", ""));

entity_ty!(
    ENTITY_TYPE_OBJECT_REMOVE_PROPERTY,
    NAMESPACE_JSON,
    ENTITY_TYPE_NAME_OBJECT_REMOVE_PROPERTY,
    "object_remove_property"
);
behaviour_ty!(
    BEHAVIOUR_OBJECT_REMOVE_PROPERTY,
    NAMESPACE_JSON,
    BEHAVIOUR_NAME_OBJECT_REMOVE_PROPERTY,
    "object_remove_property"
);
entity_behaviour_ty!(ENTITY_BEHAVIOUR_OBJECT_REMOVE_PROPERTY, ENTITY_TYPE_OBJECT_REMOVE_PROPERTY, BEHAVIOUR_OBJECT_REMOVE_PROPERTY);

entity_model!(
    ObjectRemoveProperty,
    set property_name string,
    set object object
);
impl ResultAny for ObjectRemoveProperty {}
