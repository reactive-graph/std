use std::collections::HashMap;

pub type ArithmeticOperationFunction<T> = fn(T) -> T;

pub const FN_INCREMENT: ArithmeticOperationFunction<f64> = |lhs: f64| lhs + 1.0;
pub const FN_DECREMENT: ArithmeticOperationFunction<f64> = |lhs: f64| lhs - 1.0;

lazy_static! {
    pub static ref ARITHMETIC_OPERATIONS: HashMap<&'static str, ArithmeticOperationFunction<f64>> =
        vec![("increment", FN_INCREMENT), ("decrement", FN_DECREMENT),]
            .into_iter()
            .collect();
}
