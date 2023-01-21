use serde_json::json;
use serde_json::Value;

use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::model_result::ResultArray;
use crate::NAMESPACE_JSON;

properties!(ArrayPushProperties, (ARRAY, "array", json!([])), (VALUE, "value", Value::Null));

entity_ty!(ENTITY_TYPE_ARRAY_PUSH, NAMESPACE_JSON, ENTITY_TYPE_NAME_ARRAY_PUSH, "array_push");
behaviour_ty!(BEHAVIOUR_ARRAY_PUSH, NAMESPACE_JSON, BEHAVIOUR_NAME_ARRAY_PUSH, "array_push");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_ARRAY_PUSH, ENTITY_TYPE_ARRAY_PUSH, BEHAVIOUR_ARRAY_PUSH);

entity_model!(
    ArrayPush,
    set value value,
    set array array
);
impl ResultArray for ArrayPush {}
