use crate::NAMESPACE_DATE_TIME;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;

properties!(DateComparisonProperties, (LHS, "lhs", ""), (RHS, "rhs", ""));

component_ty!(COMPONENT_DATE_COMPARISON, NAMESPACE_DATE_TIME, COMPONENT_NAME_DATE_COMPARISON, "date_comparison");

entity_model!(DateComparison, get result f64, set lhs f64, set rhs f64);
