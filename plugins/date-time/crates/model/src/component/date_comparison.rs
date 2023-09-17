use crate::NAMESPACE_DATE_TIME;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_reactive_api::entity_model;

properties!(DateComparisonProperties, (LHS, "lhs", ""), (RHS, "rhs", ""));

component_ty!(COMPONENT_DATE_COMPARISON, NAMESPACE_DATE_TIME, COMPONENT_NAME_DATE_COMPARISON, "date_comparison");

entity_model!(DateComparison, get result f64, set lhs f64, set rhs f64);
