use serde_json::Value;

use crate::NAMESPACE_RESULT;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(ResultArrayProperties, (RESULT, "result", Value::Array(Vec::new())));

component_ty!(COMPONENT_RESULT_ARRAY, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_ARRAY, "result_array");

component_model!(
    ResultArray,
    get result array
);
