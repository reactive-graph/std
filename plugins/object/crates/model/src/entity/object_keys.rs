use crate::NAMESPACE_OBJECT;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use serde_json::json;

properties!(ObjectKeysProperties, (OBJECT, "object", {}), (KEYS, "keys", json!([])));

entity_ty!(ENTITY_TYPE_OBJECT_KEYS, NAMESPACE_OBJECT, ENTITY_TYPE_NAME_OBJECT_KEYS, "object_keys");
behaviour_ty!(BEHAVIOUR_OBJECT_KEYS, NAMESPACE_OBJECT, BEHAVIOUR_NAME_OBJECT_KEYS, "object_keys");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_OBJECT_KEYS, ENTITY_TYPE_OBJECT_KEYS, BEHAVIOUR_OBJECT_KEYS);

entity_model!(
    ObjectKeys,
    get keys array,
    set object object
);
