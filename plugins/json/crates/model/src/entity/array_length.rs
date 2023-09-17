use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_reactive_api::entity_model;
use serde_json::json;

use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::NAMESPACE_JSON;

properties!(ArrayLengthProperties, (ARRAY, "array", json!([])), (LENGTH, "length", 0));

entity_ty!(ENTITY_TYPE_ARRAY_LENGTH, NAMESPACE_JSON, ENTITY_TYPE_NAME_ARRAY_LENGTH, "array_length");
behaviour_ty!(BEHAVIOUR_ARRAY_LENGTH, NAMESPACE_JSON, BEHAVIOUR_NAME_ARRAY_LENGTH, "array_length");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_ARRAY_LENGTH, ENTITY_TYPE_ARRAY_LENGTH, BEHAVIOUR_ARRAY_LENGTH);

entity_model!(
    ArrayLength,
    get length u64,
    set array array
);
