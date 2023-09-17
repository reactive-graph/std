use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_reactive_api::entity_model;
use serde_json::json;

use inexor_rgf_model_result::ResultBoolean;

use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::NAMESPACE_JSON;

properties!(ArrayContainsProperties, (ARRAY, "array", json!([])), (SEARCH, "search", false));

entity_ty!(ENTITY_TYPE_ARRAY_CONTAINS, NAMESPACE_JSON, ENTITY_TYPE_NAME_ARRAY_CONTAINS, "array_contains");
behaviour_ty!(BEHAVIOUR_ARRAY_CONTAINS, NAMESPACE_JSON, BEHAVIOUR_NAME_ARRAY_CONTAINS, "array_contains");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_ARRAY_CONTAINS, ENTITY_TYPE_ARRAY_CONTAINS, BEHAVIOUR_ARRAY_CONTAINS);

entity_model!(
    ArrayContains,
    trigger,
    set array array,
    set search value,
);
impl ResultBoolean for ArrayContains {}
