use crate::NAMESPACE_RESULT;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(ResultObjectProperties, (RESULT, "result", {}));

component_ty!(COMPONENT_RESULT_OBJECT, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_OBJECT, "result_object");

component_model!(
    ResultObject,
    get result object
);
