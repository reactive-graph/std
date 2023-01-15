use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_JSON;
use serde_json::json;

properties!(ArrayLengthProperties, (ARRAY, "array", json!([])), (LENGTH, "length", 0));

entity_ty!(ENTITY_TYPE_ARRAY_LENGTH, NAMESPACE_JSON, ENTITY_TYPE_NAME_ARRAY_LENGTH, "array_length");
behaviour_ty!(BEHAVIOUR_ARRAY_LENGTH, NAMESPACE_JSON, BEHAVIOUR_NAME_ARRAY_LENGTH, "array_length");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_ARRAY_LENGTH, ENTITY_TYPE_ARRAY_LENGTH, BEHAVIOUR_ARRAY_LENGTH);

entity_model!(
    ArrayLength,
    get length u64,
    set array array
);
