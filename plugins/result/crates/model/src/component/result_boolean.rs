use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_RESULT;

properties!(ResultBooleanProperties, (RESULT, "result", false));

component_ty!(COMPONENT_RESULT_BOOLEAN, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_BOOLEAN, "result_boolean");

component_model!(
    ResultBoolean,
    get result bool
);
