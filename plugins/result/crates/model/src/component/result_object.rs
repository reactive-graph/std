use crate::NAMESPACE_RESULT;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(ResultObjectProperties, (RESULT, "result", {}));

component_ty!(COMPONENT_RESULT_OBJECT, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_OBJECT, "result_object");

component_model!(
    ResultObject,
    get result object
);
