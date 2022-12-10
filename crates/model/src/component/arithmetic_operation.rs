use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_ARITHMETIC;

properties!(ArithmeticOperationProperties, (LHS, "lhs", false), (RESULT, "result", false));

component_ty!(
    COMPONENT_ARITHMETIC_OPERATION,
    NAMESPACE_ARITHMETIC,
    COMPONENT_NAME_ARITHMETIC_OPERATION,
    "arithmetic_operation"
);
