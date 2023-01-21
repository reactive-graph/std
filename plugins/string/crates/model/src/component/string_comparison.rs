use crate::model::component_ty;
use crate::model::entity_model;
use crate::model::properties;
use crate::model_result::ResultBoolean;
use crate::NAMESPACE_STRING;

properties!(StringComparisonProperties, (LHS, "lhs", ""), (RHS, "rhs", ""));

component_ty!(COMPONENT_STRING_COMPARISON, NAMESPACE_STRING, COMPONENT_NAME_STRING_COMPARISON, "string_comparison");

entity_model!(
    StringComparison,
    set lhs string,
    set rhs string
);
impl ResultBoolean for StringComparison {}
