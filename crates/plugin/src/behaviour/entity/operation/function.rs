use crate::model_arithmetic::NAMESPACE_ARITHMETIC_F64;
use crate::model_arithmetic::NAMESPACE_ARITHMETIC_I64;
use crate::model_arithmetic::NAMESPACE_ARITHMETIC_U64;
use crate::reactive::behaviour_functions;

pub type ArithmeticOperationFunction<T> = fn(T) -> T;
pub type ArithmeticOperationF64Function = ArithmeticOperationFunction<f64>;
pub type ArithmeticOperationI64Function = ArithmeticOperationFunction<i64>;
pub type ArithmeticOperationU64Function = ArithmeticOperationFunction<u64>;

pub const FN_INCREMENT_F64: ArithmeticOperationF64Function = |lhs| lhs + 1.0;
pub const FN_DECREMENT_F64: ArithmeticOperationF64Function = |lhs| lhs - 1.0;

pub const FN_INCREMENT_I64: ArithmeticOperationI64Function = |lhs| lhs + 1;
pub const FN_DECREMENT_I64: ArithmeticOperationI64Function = |lhs| lhs - 1;

pub const FN_INCREMENT_U64: ArithmeticOperationU64Function = |lhs| lhs + 1;
pub const FN_DECREMENT_U64: ArithmeticOperationU64Function = |lhs| lhs - 1;

behaviour_functions!(
    ARITHMETIC_OPERATIONS_F64,
    ArithmeticOperationF64Function,
    NAMESPACE_ARITHMETIC_F64,
    ("increment", FN_INCREMENT_F64),
    ("decrement", FN_DECREMENT_F64)
);

behaviour_functions!(
    ARITHMETIC_OPERATIONS_I64,
    ArithmeticOperationI64Function,
    NAMESPACE_ARITHMETIC_I64,
    ("increment", FN_INCREMENT_I64),
    ("decrement", FN_DECREMENT_I64)
);

behaviour_functions!(
    ARITHMETIC_OPERATIONS_U64,
    ArithmeticOperationU64Function,
    NAMESPACE_ARITHMETIC_U64,
    ("increment", FN_INCREMENT_U64),
    ("decrement", FN_DECREMENT_U64)
);
