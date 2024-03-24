use crate::NAMESPACE_NUMERIC;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_model_result::ResultNumberF64;
use inexor_rgf_model_result::ResultNumberI64;
use inexor_rgf_reactive_model_api::entity_model;

properties!(NumericOperationProperties, (LHS, "lhs", 0.0));

component_ty!(COMPONENT_NUMERIC_OPERATION, NAMESPACE_NUMERIC, COMPONENT_NAME_NUMERIC_OPERATION, "numeric_operation");

entity_model!(NumericOperationF64, get result f64, set lhs f64);
impl ResultNumberF64 for NumericOperationF64 {}

entity_model!(NumericOperationI64, get result i64, set lhs i64);
impl ResultNumberI64 for NumericOperationI64 {}
