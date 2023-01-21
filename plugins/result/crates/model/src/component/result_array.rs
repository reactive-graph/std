use serde_json::Value;

use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_RESULT;

properties!(ResultArrayProperties, (RESULT, "result", Value::Array(Vec::new())));

component_ty!(COMPONENT_RESULT_ARRAY, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_ARRAY, "result_array");

component_model!(
    ResultArray,
    get result array
);
