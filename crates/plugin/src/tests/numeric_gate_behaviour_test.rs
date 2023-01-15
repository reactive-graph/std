use std::sync::Arc;

use serde_json::json;

use crate::behaviour::entity::gate::function::*;
use crate::behaviour::entity::gate::properties::NumericGateProperties;
use crate::behaviour::entity::gate::NumericGate;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::ReactiveEntityInstance;
use crate::reactive::BehaviourCreationError;
use crate::reactive::Gate;
use crate::reactive::Operation;

#[test]
fn numeric_operation_behaviour_test() {
    let lhs: f64 = 0.5;
    let rhs: f64 = 0.5;

    assert_eq!(lhs.atan2(rhs), test_numeric_gate_behaviour(FN_ATAN2_F64, lhs, rhs).unwrap());
    assert_eq!(lhs.hypot(rhs), test_numeric_gate_behaviour(FN_HYPOT_F64, lhs, rhs).unwrap());
    assert_eq!(lhs.log(rhs), test_numeric_gate_behaviour(FN_LOG_F64, lhs, rhs).unwrap());
    assert_eq!(lhs.powf(rhs), test_numeric_gate_behaviour(FN_POW_F64, lhs, rhs).unwrap());
}

fn test_numeric_gate_behaviour(f: NumericGateFunction<f64>, lhs: f64, rhs: f64) -> Option<f64> {
    let b = create_numeric_gate_behaviour(f).unwrap();
    b.lhs(json!(lhs));
    b.rhs(json!(rhs));
    b.result().as_f64()
}

fn create_numeric_gate_behaviour(f: NumericGateFunction<f64>) -> Result<NumericGate<'static>, BehaviourCreationError> {
    NumericGate::new(create_numeric_gate_entity(), f)
}

fn create_numeric_gate_entity() -> Arc<ReactiveEntityInstance> {
    ReactiveEntityInstanceBuilder::new("numeric", "abs")
        .property(NumericGateProperties::LHS.as_ref(), json!(NumericGateProperties::LHS.default_value()))
        .property(NumericGateProperties::RHS.as_ref(), json!(NumericGateProperties::RHS.default_value()))
        .property(NumericGateProperties::RESULT.as_ref(), json!(NumericGateProperties::RESULT.default_value()))
        .build()
}
