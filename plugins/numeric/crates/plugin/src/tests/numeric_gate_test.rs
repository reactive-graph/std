use std::f64::consts::PI;
use std::sync::Arc;

use serde_json::json;

use crate::behaviour::entity::gate::behaviour_f64::NumericGateF64Factory;
use crate::behaviour::entity::gate::function::NUMERIC_GATES_F64;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::BehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_numeric::NumericGateF64;
use crate::model_numeric::NumericGateProperties;
use crate::model_numeric::COMPONENT_NUMERIC_GATE;
use crate::model_numeric::NAMESPACE_NUMERIC_F64;
use crate::model_result::ResultNumberF64Properties;
use crate::reactive::BehaviourFactory;

const LHS: NumericGateProperties = NumericGateProperties::LHS;
const RHS: NumericGateProperties = NumericGateProperties::RHS;
const RESULT: ResultNumberF64Properties = ResultNumberF64Properties::RESULT;

const TYPE_NAME_ATAN2: &str = "atan2";

#[test]
fn numeric_gate_behaviour_function_should_exist() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_NUMERIC_F64, TYPE_NAME_ATAN2);
    assert!(NUMERIC_GATES_F64.contains_key(&behaviour_ty));
    assert!(NUMERIC_GATES_F64.get(&behaviour_ty).is_some());
}

#[test]
fn atan2_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, TYPE_NAME_ATAN2);
    let reactive_instance = numeric_gate(&entity_ty);

    {
        let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_NUMERIC_F64, TYPE_NAME_ATAN2);
        let not_function = NUMERIC_GATES_F64.get(&behaviour_ty).expect("Failed to get function");
        let not_factory = NumericGateF64Factory::new(behaviour_ty, not_function.clone());
        let behaviour = not_factory.create(reactive_instance.clone()).expect("Failed to create behaviour");

        assert_eq!(NAMESPACE_NUMERIC_F64, behaviour.ty().namespace().as_str());
        assert_eq!(TYPE_NAME_ATAN2, behaviour.ty().type_name().as_str());

        // Set the input value
        reactive_instance.set(LHS, json!(0.0));
        reactive_instance.set(RHS, json!(0.0));
        // Expect the correct output value -> behaviour has modified the output
        assert_eq!(0.0, reactive_instance.as_f64(RESULT).unwrap());

        // Set the input value
        reactive_instance.set(LHS, json!(1.0));
        reactive_instance.set(RHS, json!(0.0));
        // Expect the correct output value -> behaviour has modified the output
        assert_eq!(PI / 2.0, reactive_instance.as_f64(RESULT).unwrap());
    }
    // The behaviour has been dropped (no more changes)
    reactive_instance.set(LHS, json!(0.0));
    reactive_instance.set(RHS, json!(0.0));
    assert_eq!(PI / 2.0, reactive_instance.as_f64(RESULT).unwrap());
}

#[test]
fn rx_atan2_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_NUMERIC_F64, TYPE_NAME_ATAN2);
    let reactive_instance = numeric_gate(&entity_ty);

    let rx_atan2 = NumericGateF64::from(reactive_instance.clone());

    {
        let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_NUMERIC_F64, TYPE_NAME_ATAN2);
        let not_function = NUMERIC_GATES_F64.get(&behaviour_ty).expect("Failed to get function");
        let not_factory = NumericGateF64Factory::new(behaviour_ty, not_function.clone());
        let behaviour = not_factory.create(reactive_instance.clone()).expect("Failed to create behaviour");

        assert_eq!(NAMESPACE_NUMERIC_F64, behaviour.ty().namespace().as_str());
        assert_eq!(TYPE_NAME_ATAN2, behaviour.ty().type_name().as_str());

        // Set the input value
        rx_atan2.lhs(0.0);
        rx_atan2.rhs(0.0);
        // Expect the correct output value -> behaviour has modified the output
        assert_eq!(0.0, rx_atan2.result().unwrap());

        // Set the input value
        rx_atan2.lhs(1.0);
        rx_atan2.rhs(0.0);
        // Expect the correct output value -> behaviour has modified the output
        assert_eq!(PI / 2.0, rx_atan2.result().unwrap());
    }
    // The behaviour has been dropped (no more changes)
    rx_atan2.lhs(0.0);
    rx_atan2.rhs(0.0);
    assert_eq!(PI / 2.0, rx_atan2.result().unwrap());
}

pub fn numeric_gate(entity_ty: &EntityTypeId) -> Arc<ReactiveEntityInstance> {
    ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(LHS, json!(0.0))
        .property(RHS, json!(0.0))
        .property(RESULT, json!(0.0))
        .component(COMPONENT_NUMERIC_GATE.clone())
        .build()
}
