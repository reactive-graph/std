use crate::model_arithmetic::NAMESPACE_ARITHMETIC_F64;
use crate::model_arithmetic::NAMESPACE_ARITHMETIC_I64;
use crate::model_arithmetic::NAMESPACE_ARITHMETIC_U64;
use crate::reactive::behaviour_functions;

pub type ArithmeticGateFunction<T> = fn(T, T) -> T;
pub type ArithmeticGateF64Function = ArithmeticGateFunction<f64>;
pub type ArithmeticGateI64Function = ArithmeticGateFunction<i64>;
pub type ArithmeticGateU64Function = ArithmeticGateFunction<u64>;

pub const FN_ADD_F64: ArithmeticGateF64Function = |lhs, rhs| lhs + rhs;
pub const FN_DIV_F64: ArithmeticGateF64Function = |lhs, rhs| if rhs != 0.0 { lhs / rhs } else { 0.0 };
pub const FN_MAX_F64: ArithmeticGateF64Function = |lhs, rhs| lhs.max(rhs);
pub const FN_MIN_F64: ArithmeticGateF64Function = |lhs, rhs| lhs.min(rhs);
pub const FN_MOD_F64: ArithmeticGateF64Function = |lhs, rhs| if rhs != 0.0 { lhs % rhs } else { 0.0 };
pub const FN_MUL_F64: ArithmeticGateF64Function = |lhs, rhs| lhs * rhs;
pub const FN_SUB_F64: ArithmeticGateF64Function = |lhs, rhs| lhs - rhs;

pub const FN_ADD_I64: ArithmeticGateI64Function = |lhs, rhs| lhs + rhs;
pub const FN_DIV_I64: ArithmeticGateI64Function = |lhs, rhs| if rhs != 0 { lhs / rhs } else { 0 };
pub const FN_MAX_I64: ArithmeticGateI64Function = |lhs, rhs| lhs.max(rhs);
pub const FN_MIN_I64: ArithmeticGateI64Function = |lhs, rhs| lhs.min(rhs);
pub const FN_MOD_I64: ArithmeticGateI64Function = |lhs, rhs| if rhs != 0 { lhs % rhs } else { 0 };
pub const FN_MUL_I64: ArithmeticGateI64Function = |lhs, rhs| lhs * rhs;
pub const FN_SUB_I64: ArithmeticGateI64Function = |lhs, rhs| lhs - rhs;

pub const FN_ADD_U64: ArithmeticGateU64Function = |lhs, rhs| lhs + rhs;
pub const FN_DIV_U64: ArithmeticGateU64Function = |lhs, rhs| if rhs != 0 { lhs / rhs } else { 0 };
pub const FN_MAX_U64: ArithmeticGateU64Function = |lhs, rhs| lhs.max(rhs);
pub const FN_MIN_U64: ArithmeticGateU64Function = |lhs, rhs| lhs.min(rhs);
pub const FN_MOD_U64: ArithmeticGateU64Function = |lhs, rhs| if rhs != 0 { lhs % rhs } else { 0 };
pub const FN_MUL_U64: ArithmeticGateU64Function = |lhs, rhs| lhs * rhs;
pub const FN_SUB_U64: ArithmeticGateU64Function = |lhs, rhs| lhs - rhs;

behaviour_functions!(
    ARITHMETIC_GATES_F64,
    ArithmeticGateF64Function,
    NAMESPACE_ARITHMETIC_F64,
    ("add", FN_ADD_F64),
    ("div", FN_DIV_F64),
    ("max", FN_MAX_F64),
    ("min", FN_MIN_F64),
    ("mod", FN_MOD_F64),
    ("mul", FN_MUL_F64),
    ("sub", FN_SUB_F64)
);

behaviour_functions!(
    ARITHMETIC_GATES_I64,
    ArithmeticGateI64Function,
    NAMESPACE_ARITHMETIC_I64,
    ("add", FN_ADD_I64),
    ("div", FN_DIV_I64),
    ("max", FN_MAX_I64),
    ("min", FN_MIN_I64),
    ("mod", FN_MOD_I64),
    ("mul", FN_MUL_I64),
    ("sub", FN_SUB_I64)
);

behaviour_functions!(
    ARITHMETIC_GATES_U64,
    ArithmeticGateU64Function,
    NAMESPACE_ARITHMETIC_U64,
    ("add", FN_ADD_U64),
    ("div", FN_DIV_U64),
    ("max", FN_MAX_U64),
    ("min", FN_MIN_U64),
    ("mod", FN_MOD_U64),
    ("mul", FN_MUL_U64),
    ("sub", FN_SUB_U64)
);
