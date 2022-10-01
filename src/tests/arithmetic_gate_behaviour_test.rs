use std::sync::Arc;

use serde_json::json;

use crate::behaviour::entity::gate::function::*;
use crate::behaviour::entity::gate::properties::ArithmeticGateProperties;
use crate::behaviour::entity::gate::ArithmeticGate;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::ReactiveEntityInstance;
use crate::reactive::Gate;
use crate::reactive::Operation;

#[test]
fn arithmetic_operation_behaviour_test() {
    let lhs: f64 = 0.5;
    let rhs: f64 = 0.5;

    assert_eq!(lhs + rhs, test_arithmetic_gate_behaviour(FN_ADD_F64, lhs, rhs).unwrap());
    assert_eq!(lhs / rhs, test_arithmetic_gate_behaviour(FN_DIV_F64, lhs, rhs).unwrap());
    assert_eq!(lhs.min(rhs), test_arithmetic_gate_behaviour(FN_MAX_F64, lhs, rhs).unwrap());
    assert_eq!(lhs.max(rhs), test_arithmetic_gate_behaviour(FN_MIN_F64, lhs, rhs).unwrap());
    assert_eq!(lhs % rhs, test_arithmetic_gate_behaviour(FN_MOD_F64, lhs, rhs).unwrap());
    assert_eq!(lhs * rhs, test_arithmetic_gate_behaviour(FN_MUL_F64, lhs, rhs).unwrap());
    assert_eq!(lhs - rhs, test_arithmetic_gate_behaviour(FN_SUB_F64, lhs, rhs).unwrap());
}

fn test_arithmetic_gate_behaviour(f: ArithmeticGateFunction<f64>, lhs: f64, rhs: f64) -> Option<f64> {
    let b = create_arithmetic_gate_behaviour(f);
    b.lhs(json!(lhs));
    b.rhs(json!(rhs));
    b.result().as_f64()
}

fn create_arithmetic_gate_behaviour(f: ArithmeticGateFunction<f64>) -> ArithmeticGate<'static> {
    ArithmeticGate::new(create_arithmetic_gate_entity(), f)
}

fn create_arithmetic_gate_entity() -> Arc<ReactiveEntityInstance> {
    ReactiveEntityInstanceBuilder::new("abs")
        .property(ArithmeticGateProperties::LHS.as_ref(), json!(ArithmeticGateProperties::LHS.default_value()))
        .property(ArithmeticGateProperties::RHS.as_ref(), json!(ArithmeticGateProperties::RHS.default_value()))
        .property(ArithmeticGateProperties::RESULT.as_ref(), json!(ArithmeticGateProperties::RESULT.default_value()))
        .build()
}
