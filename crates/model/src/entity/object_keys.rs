use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::NAMESPACE_JSON;
use serde_json::json;

properties!(ObjectKeysProperties, (OBJECT, "object", {}), (KEYS, "keys", json!([])));

entity_ty!(ENTITY_TYPE_OBJECT_KEYS, NAMESPACE_JSON, ENTITY_TYPE_NAME_OBJECT_KEYS, "object_keys");
behaviour_ty!(BEHAVIOUR_OBJECT_KEYS, NAMESPACE_JSON, BEHAVIOUR_NAME_OBJECT_KEYS, "object_keys");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_OBJECT_KEYS, ENTITY_TYPE_OBJECT_KEYS, BEHAVIOUR_OBJECT_KEYS);

entity_model!(
    ObjectKeys,
    get keys array,
    set object object
);
