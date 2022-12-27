use serde_json::json;
use serde_json::Value;

use crate::model_numeric::NAMESPACE_NUMERIC_F64;
use crate::model_numeric::NAMESPACE_NUMERIC_I64;
use crate::reactive::behaviour_functions;

pub type NumericOperationFunction<T> = fn(T) -> Value;
pub type NumericOperationF64Function = NumericOperationFunction<f64>;
pub type NumericOperationI64Function = NumericOperationFunction<i64>;

pub const FN_ABS_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.abs());
pub const FN_ACOS_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.acos());
pub const FN_ASIN_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.asin());
pub const FN_ATAN_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.atan());
pub const FN_CBRT_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.cbrt());
pub const FN_CEIL_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.ceil());
pub const FN_COS_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.cos());
pub const FN_COSH_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.cosh());
pub const FN_EXP_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.exp());
pub const FN_EXP2_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.exp2());
pub const FN_FLOOR_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.floor());
pub const FN_FRACT_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.fract());
pub const FN_LN_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.ln());
pub const FN_LOG2_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.log2());
pub const FN_LOG10_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.log10());
pub const FN_RECIP_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.recip());
pub const FN_ROUND_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.round());
pub const FN_SIGNUM_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.signum());
pub const FN_SIN_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.sin());
pub const FN_SINH_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.sinh());
pub const FN_SQRT_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.sqrt());
pub const FN_TAN_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.tan());
pub const FN_TANH_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.tanh());
pub const FN_TO_DEGREES_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.to_degrees());
pub const FN_TO_RADIANS_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.to_radians());
pub const FN_TRUNC_F64: NumericOperationF64Function = |lhs: f64| json!(lhs.trunc());

pub const FN_ABS_I64: NumericOperationI64Function = |lhs: i64| json!(lhs.abs());
pub const FN_SIGNUM_I64: NumericOperationI64Function = |lhs: i64| json!(lhs.signum());

behaviour_functions!(
    NUMERIC_OPERATIONS_F64,
    NumericOperationF64Function,
    NAMESPACE_NUMERIC_F64,
    ("abs", FN_ABS_F64),
    ("acos", FN_ACOS_F64),
    ("asin", FN_ASIN_F64),
    ("atan", FN_ATAN_F64),
    ("cbrt", FN_CBRT_F64),
    ("ceil", FN_CEIL_F64),
    ("cos", FN_COS_F64),
    ("cosh", FN_COSH_F64),
    ("exp", FN_EXP_F64),
    ("exp2", FN_EXP2_F64),
    ("floor", FN_FLOOR_F64),
    ("fract", FN_FRACT_F64),
    ("ln", FN_LN_F64),
    ("log2", FN_LOG2_F64),
    ("log10", FN_LOG10_F64),
    ("recip", FN_RECIP_F64),
    ("round", FN_ROUND_F64),
    ("signum", FN_SIGNUM_F64),
    ("sin", FN_SIN_F64),
    ("sinh", FN_SINH_F64),
    ("sqrt", FN_SQRT_F64),
    ("tan", FN_TAN_F64),
    ("tanh", FN_TANH_F64),
    ("to_degrees", FN_TO_DEGREES_F64),
    ("to_radians", FN_TO_RADIANS_F64),
    ("trunc", FN_TRUNC_F64)
);

behaviour_functions!(
    NUMERIC_OPERATIONS_I64,
    NumericOperationI64Function,
    NAMESPACE_NUMERIC_I64,
    ("abs", FN_ABS_I64),
    ("signum", FN_SIGNUM_I64)
);
