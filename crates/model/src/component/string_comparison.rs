use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::NAMESPACE_STRING;

properties!(StringComparisonProperties, (LHS, "lhs", ""), (RHS, "rhs", ""), (RESULT, "result", false));

component_ty!(COMPONENT_STRING_COMPARISON, NAMESPACE_STRING, COMPONENT_NAME_STRING_COMPARISON, "string_comparison");

entity_model!(
    StringComparison,
    get result bool,
    set lhs string,
    set rhs string
);
