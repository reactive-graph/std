use inexor_rgf_model_numeric::NAMESPACE_NUMERIC_F64;
use inexor_rgf_model_numeric::NAMESPACE_NUMERIC_I64;

use std::sync::Arc;
use std::sync::LazyLock;

use crate::behaviour::entity::operation::behaviour_f64::NumericOperationF64Factory;
use crate::behaviour::entity::operation::behaviour_i64::NumericOperationI64Factory;
use inexor_rgf_behaviour::entity::EntityBehaviourFactoryCreator;
use inexor_rgf_behaviour::entity::EntityBehaviourFunctions;
use inexor_rgf_behaviour::entity::EntityBehaviourFunctionsStorage;

pub type NumericOperationFunction<I, O> = fn(I) -> O;
pub type NumericOperationF64Function = NumericOperationFunction<f64, f64>;
pub type NumericOperationI64Function = NumericOperationFunction<i64, i64>;

pub const FN_ABS_F64: NumericOperationF64Function = |lhs: f64| lhs.abs();
pub const FN_ACOS_F64: NumericOperationF64Function = |lhs: f64| lhs.acos();
pub const FN_ASIN_F64: NumericOperationF64Function = |lhs: f64| lhs.asin();
pub const FN_ATAN_F64: NumericOperationF64Function = |lhs: f64| lhs.atan();
pub const FN_CBRT_F64: NumericOperationF64Function = |lhs: f64| lhs.cbrt();
pub const FN_CEIL_F64: NumericOperationF64Function = |lhs: f64| lhs.ceil();
pub const FN_COS_F64: NumericOperationF64Function = |lhs: f64| lhs.cos();
pub const FN_COSH_F64: NumericOperationF64Function = |lhs: f64| lhs.cosh();
pub const FN_EXP_F64: NumericOperationF64Function = |lhs: f64| lhs.exp();
pub const FN_EXP2_F64: NumericOperationF64Function = |lhs: f64| lhs.exp2();
pub const FN_FLOOR_F64: NumericOperationF64Function = |lhs: f64| lhs.floor();
pub const FN_FRACT_F64: NumericOperationF64Function = |lhs: f64| lhs.fract();
pub const FN_LN_F64: NumericOperationF64Function = |lhs: f64| lhs.ln();
pub const FN_LOG2_F64: NumericOperationF64Function = |lhs: f64| lhs.log2();
pub const FN_LOG10_F64: NumericOperationF64Function = |lhs: f64| lhs.log10();
pub const FN_RECIP_F64: NumericOperationF64Function = |lhs: f64| lhs.recip();
pub const FN_ROUND_F64: NumericOperationF64Function = |lhs: f64| lhs.round();
pub const FN_SIGNUM_F64: NumericOperationF64Function = |lhs: f64| lhs.signum();
pub const FN_SIN_F64: NumericOperationF64Function = |lhs: f64| lhs.sin();
pub const FN_SINH_F64: NumericOperationF64Function = |lhs: f64| lhs.sinh();
pub const FN_SQRT_F64: NumericOperationF64Function = |lhs: f64| lhs.sqrt();
pub const FN_TAN_F64: NumericOperationF64Function = |lhs: f64| lhs.tan();
pub const FN_TANH_F64: NumericOperationF64Function = |lhs: f64| lhs.tanh();
pub const FN_TO_DEGREES_F64: NumericOperationF64Function = |lhs: f64| lhs.to_degrees();
pub const FN_TO_RADIANS_F64: NumericOperationF64Function = |lhs: f64| lhs.to_radians();
pub const FN_TRUNC_F64: NumericOperationF64Function = |lhs: f64| lhs.trunc();

pub const FN_ABS_I64: NumericOperationI64Function = |lhs: i64| lhs.abs();
pub const FN_SIGNUM_I64: NumericOperationI64Function = |lhs: i64| lhs.signum();

const FACTORY_CREATOR_F64: EntityBehaviourFactoryCreator<NumericOperationF64Function> = |ty, f| Arc::new(NumericOperationF64Factory::new(ty.clone(), f));
const FACTORY_CREATOR_I64: EntityBehaviourFactoryCreator<NumericOperationI64Function> = |ty, f| Arc::new(NumericOperationI64Factory::new(ty.clone(), f));

pub static NUMERIC_OPERATIONS_F64: EntityBehaviourFunctionsStorage<NumericOperationF64Function> = LazyLock::new(|| {
    EntityBehaviourFunctions::<NumericOperationF64Function>::with_namespace(NAMESPACE_NUMERIC_F64, FACTORY_CREATOR_F64)
        .behaviour("abs", FN_ABS_F64)
        .behaviour("acos", FN_ACOS_F64)
        .behaviour("asin", FN_ASIN_F64)
        .behaviour("atan", FN_ATAN_F64)
        .behaviour("cbrt", FN_CBRT_F64)
        .behaviour("ceil", FN_CEIL_F64)
        .behaviour("cos", FN_COS_F64)
        .behaviour("cosh", FN_COSH_F64)
        .behaviour("exp", FN_EXP_F64)
        .behaviour("exp2", FN_EXP2_F64)
        .behaviour("floor", FN_FLOOR_F64)
        .behaviour("fract", FN_FRACT_F64)
        .behaviour("ln", FN_LN_F64)
        .behaviour("log2", FN_LOG2_F64)
        .behaviour("log10", FN_LOG10_F64)
        .behaviour("recip", FN_RECIP_F64)
        .behaviour("round", FN_ROUND_F64)
        .behaviour("signum", FN_SIGNUM_F64)
        .behaviour("sin", FN_SIN_F64)
        .behaviour("sinh", FN_SINH_F64)
        .behaviour("sqrt", FN_SQRT_F64)
        .behaviour("tan", FN_TAN_F64)
        .behaviour("tanh", FN_TANH_F64)
        .behaviour("to_degrees", FN_TO_DEGREES_F64)
        .behaviour("to_radians", FN_TO_RADIANS_F64)
        .behaviour("trunc", FN_TRUNC_F64)
        .get()
});

pub static NUMERIC_OPERATIONS_I64: EntityBehaviourFunctionsStorage<NumericOperationI64Function> = LazyLock::new(|| {
    EntityBehaviourFunctions::<NumericOperationI64Function>::with_namespace(NAMESPACE_NUMERIC_I64, FACTORY_CREATOR_I64)
        .behaviour("abs", FN_ABS_I64)
        .behaviour("signum", FN_SIGNUM_I64)
        .get()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_operation_function_test() {
        let nv: f64 = -0.5;
        let pv: f64 = 0.5;
        assert_eq!(nv.abs(), FN_ABS_F64(nv));
        assert_eq!(pv.acos(), FN_ACOS_F64(pv));
        assert_eq!(nv.asin(), FN_ASIN_F64(nv));
        assert_eq!(nv.atan(), FN_ATAN_F64(nv));
        assert_eq!(nv.cbrt(), FN_CBRT_F64(nv));
        assert_eq!(nv.ceil(), FN_CEIL_F64(nv));
        assert_eq!(nv.cos(), FN_COS_F64(nv));
        assert_eq!(nv.cosh(), FN_COSH_F64(nv));
        assert_eq!(nv.exp(), FN_EXP_F64(nv));
        assert_eq!(nv.exp2(), FN_EXP2_F64(nv));
        assert_eq!(nv.floor(), FN_FLOOR_F64(nv));
        assert_eq!(nv.fract(), FN_FRACT_F64(nv));
        assert_eq!(pv.ln(), FN_LN_F64(pv));
        assert_eq!(pv.log2(), FN_LOG2_F64(pv));
        assert_eq!(pv.log10(), FN_LOG10_F64(pv));
        assert_eq!(nv.recip(), FN_RECIP_F64(nv));
        assert_eq!(nv.round(), FN_ROUND_F64(nv));
        assert_eq!(nv.signum(), FN_SIGNUM_F64(nv));
        assert_eq!(nv.sin(), FN_SIN_F64(nv));
        assert_eq!(nv.sinh(), FN_SINH_F64(nv));
        assert_eq!(pv.sqrt(), FN_SQRT_F64(pv));
        assert_eq!(nv.tan(), FN_TAN_F64(nv));
        assert_eq!(nv.tanh(), FN_TANH_F64(nv));
        assert_eq!(nv.to_degrees(), FN_TO_DEGREES_F64(nv));
        assert_eq!(nv.to_radians(), FN_TO_RADIANS_F64(nv));
        assert_eq!(nv.trunc(), FN_TRUNC_F64(nv));
    }
}
