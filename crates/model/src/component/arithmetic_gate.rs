use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_ARITHMETIC;

properties!(ArithmeticGateProperties, (LHS, "lhs", false), (RHS, "rhs", false), (RESULT, "result", false));

component_ty!(COMPONENT_ARITHMETIC_GATE, NAMESPACE_ARITHMETIC, COMPONENT_NAME_ARITHMETIC_GATE, "arithmetic_gate");
