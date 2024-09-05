use crate::NAMESPACE_RESULT;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(ResultBooleanProperties, (RESULT, "result", false));

component_ty!(COMPONENT_RESULT_BOOLEAN, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_BOOLEAN, "result_boolean");

component_model!(
    ResultBoolean,
    get result bool
);
