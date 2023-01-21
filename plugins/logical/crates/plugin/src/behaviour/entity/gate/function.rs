use crate::model_logical::NAMESPACE_LOGICAL;
use crate::reactive::behaviour_functions;

pub type LogicalGateFunction = fn(bool, bool) -> bool;

pub const FN_AND: LogicalGateFunction = |lhs, rhs| lhs && rhs;
pub const FN_NAND: LogicalGateFunction = |lhs, rhs| !(lhs && rhs);
pub const FN_NOR: LogicalGateFunction = |lhs, rhs| !(lhs || rhs);
pub const FN_OR: LogicalGateFunction = |lhs, rhs| lhs || rhs;
pub const FN_XOR: LogicalGateFunction = |lhs, rhs| lhs ^ rhs;
pub const FN_XNOR: LogicalGateFunction = |lhs, rhs| !(lhs ^ rhs);

behaviour_functions!(
    LOGICAL_GATES,
    LogicalGateFunction,
    NAMESPACE_LOGICAL,
    ("and", FN_AND),
    ("nand", FN_NAND),
    ("nor", FN_NOR),
    ("or", FN_OR),
    ("xor", FN_XOR),
    ("xnor", FN_XNOR)
);
