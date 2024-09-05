use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use serde_json::json;

use reactive_graph_model_result::ResultAny;

use crate::NAMESPACE_JSON;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;

properties!(ArrayGetByIndexProperties, (ARRAY, "array", json!([])), (INDEX, "index", 0));

entity_ty!(ENTITY_TYPE_ARRAY_GET_BY_INDEX, NAMESPACE_JSON, ENTITY_TYPE_NAME_ARRAY_GET_BY_INDEX, "array_get_by_index");
behaviour_ty!(BEHAVIOUR_ARRAY_GET_BY_INDEX, NAMESPACE_JSON, BEHAVIOUR_NAME_ARRAY_GET_BY_INDEX, "array_get_by_index");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_ARRAY_GET_BY_INDEX, ENTITY_TYPE_ARRAY_GET_BY_INDEX, BEHAVIOUR_ARRAY_GET_BY_INDEX);

entity_model!(
    ArrayGetByIndex,
    set array array,
    set index u64,
);
impl ResultAny for ArrayGetByIndex {}
