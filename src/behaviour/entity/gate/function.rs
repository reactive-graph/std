use std::collections::HashMap;

// TODO: Floating Point / Integer
// TODO: Different types for input and output (e.g. abs)
pub type NumericGateFunction<T> = fn(T, T) -> T;

pub const FN_ATAN2_F64: NumericGateFunction<f64> = |lhs, rhs| lhs.atan2(rhs);
pub const FN_HYPOT_F64: NumericGateFunction<f64> = |lhs, rhs| lhs.hypot(rhs);
pub const FN_LOG_F64: NumericGateFunction<f64> = |lhs, rhs| lhs.log(rhs);
pub const FN_POW_F64: NumericGateFunction<f64> = |lhs, rhs| lhs.powf(rhs);

lazy_static! {
    pub static ref NUMERIC_GATES: HashMap<&'static str, NumericGateFunction<f64>> = vec![
        ("atan2", FN_ATAN2_F64),
        ("hypot", FN_HYPOT_F64),
        ("log", FN_POW_F64),
        ("pow", FN_POW_F64),
    ]
    .into_iter()
    .collect();
}
