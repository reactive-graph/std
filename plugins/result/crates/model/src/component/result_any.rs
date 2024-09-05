use crate::NAMESPACE_RESULT;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(ResultAnyProperties, (RESULT, "result", {}));

component_ty!(COMPONENT_RESULT_ANY, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_ANY, "result_any");

component_model!(
    ResultAny,
    get result value
);
