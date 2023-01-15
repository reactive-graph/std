use crate::model_logical::NAMESPACE_LOGICAL;
use crate::reactive::behaviour_functions;

pub type LogicalOperationFunction = fn(bool) -> bool;

pub const FN_NOT: LogicalOperationFunction = |lhs| !lhs;

behaviour_functions!(LOGICAL_OPERATIONS, LogicalOperationFunction, NAMESPACE_LOGICAL, ("not", FN_NOT));
