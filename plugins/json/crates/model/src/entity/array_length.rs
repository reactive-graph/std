use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use serde_json::json;

use crate::NAMESPACE_JSON;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;

properties!(ArrayLengthProperties, (ARRAY, "array", json!([])), (LENGTH, "length", 0));

entity_ty!(ENTITY_TYPE_ARRAY_LENGTH, NAMESPACE_JSON, ENTITY_TYPE_NAME_ARRAY_LENGTH, "array_length");
behaviour_ty!(BEHAVIOUR_ARRAY_LENGTH, NAMESPACE_JSON, BEHAVIOUR_NAME_ARRAY_LENGTH, "array_length");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_ARRAY_LENGTH, ENTITY_TYPE_ARRAY_LENGTH, BEHAVIOUR_ARRAY_LENGTH);

entity_model!(
    ArrayLength,
    get length u64,
    set array array
);
