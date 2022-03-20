use std::collections::HashMap;

// TODO: Floating Point / Integer
pub type ArithmeticGateFunction<T> = fn(T, T) -> T;

pub const FN_ADD_F64: ArithmeticGateFunction<f64> = |lhs, rhs| lhs + rhs;
pub const FN_DIV_F64: ArithmeticGateFunction<f64> = |lhs, rhs| if rhs != 0.0 { lhs / rhs } else { 0.0 };
pub const FN_MAX_F64: ArithmeticGateFunction<f64> = |lhs, rhs| lhs.max(rhs);
pub const FN_MIN_F64: ArithmeticGateFunction<f64> = |lhs, rhs| lhs.min(rhs);
pub const FN_MOD_F64: ArithmeticGateFunction<f64> = |lhs, rhs| if rhs != 0.0 { lhs % rhs } else { 0.0 };
pub const FN_MUL_F64: ArithmeticGateFunction<f64> = |lhs, rhs| lhs * rhs;
pub const FN_SUB_F64: ArithmeticGateFunction<f64> = |lhs, rhs| lhs - rhs;

lazy_static! {
    pub static ref ARITHMETIC_GATES: HashMap<&'static str, ArithmeticGateFunction<f64>> = vec![
        ("add", FN_ADD_F64),
        ("div", FN_DIV_F64),
        ("max", FN_MAX_F64),
        ("min", FN_MIN_F64),
        ("mod", FN_MOD_F64),
        ("mul", FN_MUL_F64),
        ("sub", FN_SUB_F64),
    ]
    .into_iter()
    .collect();
}
