use std::sync::Arc;

use serde_json::json;

use crate::behaviour::entity::operation::function::*;
use crate::behaviour::entity::operation::NumericOperation;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::ReactiveEntityInstance;
use crate::reactive::Operation;
use crate::NumericOperationProperties;

#[test]
fn numeric_operation_behaviour_test() {
    let nv: f64 = -0.5;
    let pv: f64 = 0.5;

    assert_eq!(nv.abs(), test_numeric_operation_behaviour(FN_ABS_F64, nv).unwrap());
    assert_eq!(pv.acos(), test_numeric_operation_behaviour(FN_ACOS_F64, pv).unwrap());
    assert_eq!(nv.asin(), test_numeric_operation_behaviour(FN_ASIN_F64, nv).unwrap());
    assert_eq!(nv.atan(), test_numeric_operation_behaviour(FN_ATAN_F64, nv).unwrap());
    assert_eq!(nv.cbrt(), test_numeric_operation_behaviour(FN_CBRT_F64, nv).unwrap());
    assert_eq!(nv.ceil(), test_numeric_operation_behaviour(FN_CEIL_F64, nv).unwrap());
    assert_eq!(nv.cos(), test_numeric_operation_behaviour(FN_COS_F64, nv).unwrap());
    assert_eq!(nv.cosh(), test_numeric_operation_behaviour(FN_COSH_F64, nv).unwrap());
    assert_eq!(nv.exp(), test_numeric_operation_behaviour(FN_EXP_F64, nv).unwrap());
    assert_eq!(nv.exp2(), test_numeric_operation_behaviour(FN_EXP2_F64, nv).unwrap());
    assert_eq!(nv.floor(), test_numeric_operation_behaviour(FN_FLOOR_F64, nv).unwrap());
    assert_eq!(nv.fract(), test_numeric_operation_behaviour(FN_FRACT_F64, nv).unwrap());
    assert_eq!(pv.ln(), test_numeric_operation_behaviour(FN_LN_F64, pv).unwrap());
    assert_eq!(pv.log2(), test_numeric_operation_behaviour(FN_LOG2_F64, pv).unwrap());
    assert_eq!(pv.log10(), test_numeric_operation_behaviour(FN_LOG10_F64, pv).unwrap());
    assert_eq!(nv.recip(), test_numeric_operation_behaviour(FN_RECIP_F64, nv).unwrap());
    assert_eq!(nv.round(), test_numeric_operation_behaviour(FN_ROUND_F64, nv).unwrap());
    assert_eq!(nv.signum(), test_numeric_operation_behaviour(FN_SIGNUM_F64, nv).unwrap());
    assert_eq!(nv.sin(), test_numeric_operation_behaviour(FN_SIN_F64, nv).unwrap());
    assert_eq!(nv.sinh(), test_numeric_operation_behaviour(FN_SINH_F64, nv).unwrap());
    assert_eq!(pv.sqrt(), test_numeric_operation_behaviour(FN_SQRT_F64, pv).unwrap());
    assert_eq!(nv.tan(), test_numeric_operation_behaviour(FN_TAN_F64, nv).unwrap());
    assert_eq!(nv.tanh(), test_numeric_operation_behaviour(FN_TANH_F64, nv).unwrap());
    assert_eq!(nv.to_degrees(), test_numeric_operation_behaviour(FN_TO_DEGREES_F64, nv).unwrap());
    assert_eq!(nv.to_radians(), test_numeric_operation_behaviour(FN_TO_RADIANS_F64, nv).unwrap());
    assert_eq!(nv.trunc(), test_numeric_operation_behaviour(FN_TRUNC_F64, nv).unwrap());
}

fn test_numeric_operation_behaviour(f: NumericOperationFunction<f64>, v: f64) -> Option<f64> {
    let b = create_numeric_operation_behaviour(f);
    b.lhs(json!(v));
    b.result().as_f64()
}

fn create_numeric_operation_behaviour(f: NumericOperationFunction<f64>) -> NumericOperation<'static> {
    NumericOperation::new(create_numeric_operation_entity(), f)
}

fn create_numeric_operation_entity() -> Arc<ReactiveEntityInstance> {
    ReactiveEntityInstanceBuilder::new("abs")
        .property(NumericOperationProperties::LHS.as_ref(), json!(NumericOperationProperties::LHS.default_value()))
        .property(NumericOperationProperties::RESULT.as_ref(), json!(NumericOperationProperties::RESULT.default_value()))
        .build()
}
