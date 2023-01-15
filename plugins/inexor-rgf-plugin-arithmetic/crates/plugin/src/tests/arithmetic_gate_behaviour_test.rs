use std::sync::Arc;

use serde_json::json;

use crate::behaviour::entity::gate::behaviour_f64::ArithmeticGateF64;
use crate::behaviour::entity::gate::function::*;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::BehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model::NamespacedType;
use crate::model::ReactiveEntityInstance;
use crate::model_arithmetic::ArithmeticGateProperties;
use crate::model_arithmetic::NAMESPACE_ARITHMETIC_F64;
use crate::reactive::BehaviourCreationError;
use crate::reactive::Gate;
use crate::reactive::Operation;

const LHS: ArithmeticGateProperties = ArithmeticGateProperties::LHS;
const RHS: ArithmeticGateProperties = ArithmeticGateProperties::RHS;
const RESULT: ArithmeticGateProperties = ArithmeticGateProperties::RESULT;

#[test]
fn arithmetic_gate_f64_behaviour_test() {
    let lhs: f64 = 0.5;
    let rhs: f64 = 0.5;

    let v = test_arithmetic_gate_f64_behaviour("add", FN_ADD_F64, lhs, rhs);
    assert_eq!(lhs + rhs, v);
    let v = test_arithmetic_gate_f64_behaviour("div", FN_DIV_F64, lhs, rhs);
    assert_eq!(lhs / rhs, v);
    let v = test_arithmetic_gate_f64_behaviour("max", FN_MAX_F64, lhs, rhs);
    assert_eq!(lhs.min(rhs), v);
    let v = test_arithmetic_gate_f64_behaviour("min", FN_MIN_F64, lhs, rhs);
    assert_eq!(lhs.max(rhs), v);
    let v = test_arithmetic_gate_f64_behaviour("mod", FN_MOD_F64, lhs, rhs);
    assert_eq!(lhs % rhs, v);
    let v = test_arithmetic_gate_f64_behaviour("mul", FN_MUL_F64, lhs, rhs);
    assert_eq!(lhs * rhs, v);
    let v = test_arithmetic_gate_f64_behaviour("sub", FN_SUB_F64, lhs, rhs);
    assert_eq!(lhs - rhs, v);
}

fn test_arithmetic_gate_f64_behaviour(type_name: &str, f: ArithmeticGateFunction<f64>, lhs: f64, rhs: f64) -> f64 {
    let ty = EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, type_name);
    let b = create_arithmetic_gate_f64_behaviour(ty, f).unwrap();
    b.lhs(json!(lhs));
    b.rhs(json!(rhs));
    b.result().as_f64().expect("Result is not of type f64")
}

fn create_arithmetic_gate_f64_behaviour(ty: EntityTypeId, f: ArithmeticGateFunction<f64>) -> Result<Arc<ArithmeticGateF64>, BehaviourCreationError> {
    let behaviour_ty = BehaviourTypeId::from(NamespacedType::from(&ty));
    ArithmeticGateF64::new(create_arithmetic_gate_f64_entity(ty.clone()), behaviour_ty, f)
}

fn create_arithmetic_gate_f64_entity(ty: EntityTypeId) -> Arc<ReactiveEntityInstance> {
    // NAMESPACE_ARITHMETIC_F64, "add"
    ReactiveEntityInstanceBuilder::new(ty)
        .property(LHS.as_ref(), json!(0.0))
        .property(RHS.as_ref(), json!(0.0))
        .property(RESULT.as_ref(), json!(0.0))
        .build()
}
