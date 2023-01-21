use serde_json::json;

use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::model_result::ResultAny;
use crate::NAMESPACE_JSON;

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
