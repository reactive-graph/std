use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_RESULT;

properties!(ResultAnyProperties, (RESULT, "result", {}));

component_ty!(COMPONENT_RESULT_ANY, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_ANY, "result_any");

component_model!(
    ResultAny,
    get result value
);
