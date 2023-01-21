use std::sync::Arc;

use serde_json::json;

use crate::behaviour::entity::operation::behaviour_f64::ArithmeticOperationF64;
use crate::behaviour::entity::operation::function::*;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::BehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model::NamespacedType;
use crate::model::ReactiveEntityInstance;
use crate::model_arithmetic::ArithmeticOperationProperties;
use crate::model_arithmetic::NAMESPACE_ARITHMETIC_F64;
use crate::reactive::BehaviourReactiveInstanceContainer;

const LHS: ArithmeticOperationProperties = ArithmeticOperationProperties::LHS;
const RESULT: ArithmeticOperationProperties = ArithmeticOperationProperties::RESULT;

#[test]
fn arithmetic_operation_behaviour_test() {
    let lhs: f64 = -10.0;
    let incremented_value = test_arithmetic_operation_behaviour("increment", FN_INCREMENT_F64, lhs);
    assert_eq!(-9.0, incremented_value);
    let decremented_value = test_arithmetic_operation_behaviour("decrement", FN_DECREMENT_F64, lhs);
    assert_eq!(-11.0, decremented_value);
}

fn test_arithmetic_operation_behaviour(type_name: &str, f: ArithmeticOperationFunction<f64>, v: f64) -> f64 {
    let ty = EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, type_name);
    let behaviour = create_arithmetic_operation_behaviour(ty, f);
    let reactive_instance = behaviour.get_reactive_instance();
    let arithmetic_operation = crate::model_arithmetic::ArithmeticOperationF64::from(reactive_instance.clone());
    arithmetic_operation.lhs(v);
    arithmetic_operation.result().expect("Result is not of type f64")
}

fn create_arithmetic_operation_behaviour(ty: EntityTypeId, f: ArithmeticOperationFunction<f64>) -> Arc<ArithmeticOperationF64> {
    let behaviour_ty = BehaviourTypeId::from(NamespacedType::from(&ty));
    let reactive_instance = create_arithmetic_operation_entity(ty.clone());
    ArithmeticOperationF64::new(reactive_instance, behaviour_ty, f).expect("Failed to create ArithmeticOperationF64")
}

fn create_arithmetic_operation_entity(ty: EntityTypeId) -> Arc<ReactiveEntityInstance> {
    ReactiveEntityInstanceBuilder::new(ty)
        .property(LHS, json!(0.0))
        .property(RESULT, json!(0.0))
        .build()
}
