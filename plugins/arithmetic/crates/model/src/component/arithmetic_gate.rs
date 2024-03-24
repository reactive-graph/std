use crate::NAMESPACE_ARITHMETIC;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_reactive_model_api::entity_model;

properties!(ArithmeticGateProperties, (LHS, "lhs", false), (RHS, "rhs", false), (RESULT, "result", false));

component_ty!(COMPONENT_ARITHMETIC_GATE, NAMESPACE_ARITHMETIC, COMPONENT_NAME_ARITHMETIC_GATE, "arithmetic_gate");

entity_model!(ArithmeticGateF64, get result f64, set lhs f64, set rhs f64);
entity_model!(ArithmeticGateI64, get result i64, set lhs i64, set rhs i64);
entity_model!(ArithmeticGateU64, get result u64, set lhs u64, set rhs u64);
