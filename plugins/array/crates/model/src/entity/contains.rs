use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use serde_json::json;

use reactive_graph_std_result_model::ResultBoolean;

use crate::NAMESPACE_ARRAY;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;

properties!(ArrayContainsProperties, (ARRAY, "array", json!([])), (SEARCH, "search", false));

entity_ty!(ENTITY_TYPE_ARRAY_CONTAINS, NAMESPACE_ARRAY, ENTITY_TYPE_NAME_ARRAY_CONTAINS, "contains");
behaviour_ty!(BEHAVIOUR_ARRAY_CONTAINS, NAMESPACE_ARRAY, BEHAVIOUR_NAME_ARRAY_CONTAINS, "contains");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_ARRAY_CONTAINS, ENTITY_TYPE_ARRAY_CONTAINS, BEHAVIOUR_ARRAY_CONTAINS);

entity_model!(
    ArrayContains,
    trigger,
    set array array,
    set search value,
);
impl ResultBoolean for ArrayContains {}
