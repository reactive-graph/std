use crate::NAMESPACE_RESULT;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(ResultAnyProperties, (RESULT, "result", {}));

component_ty!(COMPONENT_RESULT_ANY, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_ANY, "result_any");

component_model!(
    ResultAny,
    get result value
);
