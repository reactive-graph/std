use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_RESULT;

properties!(ResultStringProperties, (RESULT, "result", ""));

component_ty!(COMPONENT_RESULT_STRING, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_STRING, "result_string");

component_model!(
    ResultString,
    get result string
);
