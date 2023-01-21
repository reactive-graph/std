use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_RESULT;

properties!(ResultNumberF64Properties, (RESULT, "result", 0.0));
properties!(ResultNumberI64Properties, (RESULT, "result", 0));
properties!(ResultNumberU64Properties, (RESULT, "result", 0));

component_ty!(COMPONENT_RESULT_NUMBER, NAMESPACE_RESULT, COMPONENT_NAME_RESULT_NUMBER, "result_number");

component_model!(
    ResultNumberF64,
    get result f64
);

component_model!(
    ResultNumberI64,
    get result i64
);

component_model!(
    ResultNumberU64,
    get result u64
);
