use std::f64::consts::PI;
use std::sync::Arc;

use serde_json::json;

use crate::behaviour::entity::operation::behaviour_f64::NumericOperationF64Factory;
use crate::behaviour::entity::operation::function::NUMERIC_OPERATIONS_F64;
use crate::behaviour::entity::operation::function::NUMERIC_OPERATIONS_I64;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::BehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_numeric::NumericOperationF64;
use crate::model_numeric::NumericOperationProperties;
use crate::model_numeric::COMPONENT_NUMERIC_OPERATION;
use crate::model_numeric::NAMESPACE_NUMERIC_F64;
use crate::model_numeric::NAMESPACE_NUMERIC_I64;
use crate::model_result::ResultNumberF64Properties;
use crate::reactive::BehaviourFactory;

const LHS: NumericOperationProperties = NumericOperationProperties::LHS;
const RESULT: ResultNumberF64Properties = ResultNumberF64Properties::RESULT;

const TYPE_NAME_SIN: &str = "sin";

const TYPE_NAME_ABS: &str = "abs";

#[test]
fn numeric_operation_behaviour_function_should_exist() {
    // f64
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_NUMERIC_F64, TYPE_NAME_SIN);
    assert!(NUMERIC_OPERATIONS_F64.contains_key(&behaviour_ty));
    assert!(NUMERIC_OPERATIONS_F64.get(&behaviour_ty).is_some());
    // i64
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_NUMERIC_I64, TYPE_NAME_ABS);
    assert!(NUMERIC_OPERATIONS_I64.contains_key(&behaviour_ty));
    assert!(NUMERIC_OPERATIONS_I64.get(&behaviour_ty).is_some());
}

#[test]
fn sin_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, TYPE_NAME_SIN);
    let reactive_instance = numeric_operation(&entity_ty);

    {
        let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_NUMERIC_F64, TYPE_NAME_SIN);
        let not_function = NUMERIC_OPERATIONS_F64.get(&behaviour_ty).expect("Failed to get function");
        let not_factory = NumericOperationF64Factory::new(behaviour_ty, not_function.clone());
        let behaviour = not_factory.create(reactive_instance.clone()).expect("Failed to create behaviour");

        assert_eq!(NAMESPACE_NUMERIC_F64, behaviour.ty().namespace().as_str());
        assert_eq!(TYPE_NAME_SIN, behaviour.ty().type_name().as_str());

        // Set the input value
        reactive_instance.set(LHS, json!(0.0));
        // Expect the correct output value -> behaviour has modified the output
        assert_eq!(0.0, reactive_instance.as_f64(RESULT).unwrap());

        // Set the input value
        reactive_instance.set(LHS, json!(PI / 2.0));
        // Expect the correct output value -> behaviour has modified the output
        assert_eq!(1.0, reactive_instance.as_f64(RESULT).unwrap());
    }
    // The behaviour has been dropped (no more changes)
    reactive_instance.set(LHS, json!(0.0));
    assert_eq!(1.0, reactive_instance.as_f64(RESULT).unwrap());
}

#[test]
fn rx_sin_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, TYPE_NAME_SIN);
    let reactive_instance = numeric_operation(&entity_ty);

    let rx_sin = NumericOperationF64::from(reactive_instance.clone());

    {
        let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_NUMERIC_F64, TYPE_NAME_SIN);
        let not_function = NUMERIC_OPERATIONS_F64.get(&behaviour_ty).expect("Failed to get function");
        let not_factory = NumericOperationF64Factory::new(behaviour_ty, not_function.clone());
        let behaviour = not_factory.create(reactive_instance.clone()).expect("Failed to create behaviour");

        assert_eq!(NAMESPACE_NUMERIC_F64, behaviour.ty().namespace().as_str());
        assert_eq!(TYPE_NAME_SIN, behaviour.ty().type_name().as_str());

        // Set the input value
        rx_sin.lhs(0.0);
        // Expect the correct output value -> behaviour has modified the output
        assert_eq!(0.0, rx_sin.result().unwrap());

        // Set the input value
        rx_sin.lhs(PI / 2.0);
        // Expect the correct output value -> behaviour has modified the output
        assert_eq!(1.0, rx_sin.result().unwrap());
    }
    // The behaviour has been dropped (no more changes)
    rx_sin.lhs(0.0);
    assert_eq!(1.0, rx_sin.result().unwrap());
}

pub fn numeric_operation(entity_ty: &EntityTypeId) -> Arc<ReactiveEntityInstance> {
    ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(LHS, json!(0.0))
        .property(RESULT, json!(0.0))
        .component(COMPONENT_NUMERIC_OPERATION.clone())
        .build()
}
