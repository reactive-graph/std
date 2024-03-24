use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_reactive_model_api::entity_model;
use serde_json::json;
use serde_json::Value;

use inexor_rgf_model_result::ResultArray;

use crate::NAMESPACE_JSON;
use inexor_rgf_behaviour_model_api::behaviour_ty;
use inexor_rgf_behaviour_model_api::entity_behaviour_ty;

properties!(ArrayPopProperties, (ARRAY, "array", json!([])), (VALUE, "value", Value::Null));

entity_ty!(ENTITY_TYPE_ARRAY_POP, NAMESPACE_JSON, ENTITY_TYPE_NAME_ARRAY_POP, "array_pop");
behaviour_ty!(BEHAVIOUR_ARRAY_POP, NAMESPACE_JSON, BEHAVIOUR_NAME_ARRAY_POP, "array_pop");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_ARRAY_POP, ENTITY_TYPE_ARRAY_POP, BEHAVIOUR_ARRAY_POP);

entity_model!(
    ArrayPop,
    get result array,
    get value value,
    set array array
);
impl ResultArray for ArrayPop {}
