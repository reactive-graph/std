use crate::NAMESPACE_NUMERIC;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_std_result_model::ResultNumberF64;
use reactive_graph_std_result_model::ResultNumberI64;

properties!(NumericGateProperties, (LHS, "lhs", 0.0), (RHS, "rhs", 0.0));

component_ty!(COMPONENT_NUMERIC_GATE, NAMESPACE_NUMERIC, COMPONENT_NAME_NUMERIC_GATE, "numeric_gate");

entity_model!(NumericGateF64, get result f64, set lhs f64, set rhs f64);
impl ResultNumberF64 for NumericGateF64 {}

entity_model!(NumericGateI64, get result i64, set lhs i64, set rhs i64);
impl ResultNumberI64 for NumericGateI64 {}
