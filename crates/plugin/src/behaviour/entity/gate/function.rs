use serde_json::json;
use serde_json::Value;

use crate::model_numeric::NAMESPACE_NUMERIC_F64;
use crate::reactive::behaviour_functions;

pub type NumericGateFunction<T> = fn(T, T) -> Value;
pub type NumericGateF64Function = NumericGateFunction<f64>;

pub const FN_ATAN2_F64: NumericGateF64Function = |lhs, rhs| json!(lhs.atan2(rhs));
pub const FN_HYPOT_F64: NumericGateF64Function = |lhs, rhs| json!(lhs.hypot(rhs));
pub const FN_LOG_F64: NumericGateF64Function = |lhs, rhs| json!(lhs.log(rhs));
pub const FN_POW_F64: NumericGateF64Function = |lhs, rhs| json!(lhs.powf(rhs));

behaviour_functions!(
    NUMERIC_GATES_F64,
    NumericGateF64Function,
    NAMESPACE_NUMERIC_F64,
    ("atan2", FN_ATAN2_F64),
    ("hypot", FN_HYPOT_F64),
    ("log", FN_POW_F64),
    ("pow", FN_POW_F64)
);
