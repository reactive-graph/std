use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_reactive_model_api::entity_model;
use serde_json::json;

use inexor_rgf_model_result::ResultArray;

use crate::NAMESPACE_JSON;
use inexor_rgf_behaviour_model_api::behaviour_ty;
use inexor_rgf_behaviour_model_api::entity_behaviour_ty;

properties!(ArrayReverseProperties, (ARRAY, "array", json!([])));

entity_ty!(ENTITY_TYPE_ARRAY_REVERSE, NAMESPACE_JSON, ENTITY_TYPE_NAME_ARRAY_REVERSE, "array_reverse");
behaviour_ty!(BEHAVIOUR_ARRAY_REVERSE, NAMESPACE_JSON, BEHAVIOUR_NAME_ARRAY_REVERSE, "array_reverse");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_ARRAY_REVERSE, ENTITY_TYPE_ARRAY_REVERSE, BEHAVIOUR_ARRAY_REVERSE);

entity_model!(
    ArrayReverse,
    set array array
);
impl ResultArray for ArrayReverse {}
