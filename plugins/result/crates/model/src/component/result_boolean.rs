use crate::NAMESPACE_RESULT;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(ResultBooleanProperties, (RESULT, "result", false));

component_ty!(COMPONENT_RESULT_BOOLEAN, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_BOOLEAN, "result_boolean");

component_model!(
    ResultBoolean,
    get result bool
);
