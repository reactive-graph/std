use serde_json::Value;

use crate::NAMESPACE_RESULT;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(ResultArrayProperties, (RESULT, "result", Value::Array(Vec::new())));

component_ty!(COMPONENT_RESULT_ARRAY, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_ARRAY, "result_array");

component_model!(
    ResultArray,
    get result array
);
