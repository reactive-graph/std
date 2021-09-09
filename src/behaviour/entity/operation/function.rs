use std::collections::HashMap;

pub type LogicalOperationFunction = fn(bool) -> bool;

pub const FN_NOT: LogicalOperationFunction = |lhs| !lhs;

lazy_static! {
    pub static ref LOGICAL_OPERATIONS: HashMap<&'static str, LogicalOperationFunction> =
        vec![("not", FN_NOT),].into_iter().collect();
}
