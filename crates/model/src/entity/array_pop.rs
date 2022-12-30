use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model::properties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::NAMESPACE_JSON;
use serde_json::json;
use serde_json::Value;

properties!(ArrayPopProperties, (ARRAY, "array", json!([])), (RESULT, "result", json!([])), (VALUE, "value", Value::Null));

entity_ty!(ENTITY_TYPE_ARRAY_POP, NAMESPACE_JSON, ENTITY_TYPE_NAME_ARRAY_POP, "array_pop");
behaviour_ty!(BEHAVIOUR_ARRAY_POP, NAMESPACE_JSON, BEHAVIOUR_NAME_ARRAY_POP, "array_pop");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_ARRAY_POP, ENTITY_TYPE_ARRAY_POP, BEHAVIOUR_ARRAY_POP);

entity_model!(
    ArrayPop,
    get result array,
    get value value,
    set array array
);
