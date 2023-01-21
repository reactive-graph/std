use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_RESULT;

properties!(ResultObjectProperties, (RESULT, "result", {}));

component_ty!(COMPONENT_RESULT_OBJECT, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_OBJECT, "result_object");

component_model!(
    ResultObject,
    get result object
);
