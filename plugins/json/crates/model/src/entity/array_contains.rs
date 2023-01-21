use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_JSON;
use inexor_rgf_model_result::ResultBoolean;
use serde_json::json;

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
