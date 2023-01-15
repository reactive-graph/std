use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::NAMESPACE_ARITHMETIC;

properties!(ArithmeticOperationProperties, (LHS, "lhs", false), (RESULT, "result", false));

component_ty!(
    COMPONENT_ARITHMETIC_OPERATION,
    NAMESPACE_ARITHMETIC,
    COMPONENT_NAME_ARITHMETIC_OPERATION,
    "arithmetic_operation"
);

entity_model!(ArithmeticOperationF64, get result f64, set lhs f64);
entity_model!(ArithmeticOperationI64, get result i64, set lhs i64);
entity_model!(ArithmeticOperationU64, get result u64, set lhs u64);
