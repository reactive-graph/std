use crate::NAMESPACE_RESULT;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(ResultStringProperties, (RESULT, "result", ""));

component_ty!(COMPONENT_RESULT_STRING, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_STRING, "result_string");

component_model!(
    ResultString,
    get result string
);
