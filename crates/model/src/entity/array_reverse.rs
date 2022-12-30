use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::NAMESPACE_JSON;
use serde_json::json;

properties!(ArrayReverseProperties, (ARRAY, "array", json!([])), (RESULT, "result", json!([])));

entity_ty!(ENTITY_TYPE_ARRAY_REVERSE, NAMESPACE_JSON, ENTITY_TYPE_NAME_ARRAY_REVERSE, "array_reverse");
behaviour_ty!(BEHAVIOUR_ARRAY_REVERSE, NAMESPACE_JSON, BEHAVIOUR_NAME_ARRAY_REVERSE, "array_reverse");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_ARRAY_REVERSE, ENTITY_TYPE_ARRAY_REVERSE, BEHAVIOUR_ARRAY_REVERSE);

entity_model!(
    ArrayReverse,
    get result array,
    set array array
);
