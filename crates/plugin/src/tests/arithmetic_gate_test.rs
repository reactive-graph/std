use std::sync::Arc;

use serde_json::json;

use crate::behaviour::entity::gate::behaviour_f64::ArithmeticGateF64Factory;
use crate::behaviour::entity::gate::function::ARITHMETIC_GATES_F64;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::BehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model_arithmetic::ArithmeticGateF64;
use crate::model_arithmetic::ArithmeticGateProperties;
use crate::model_arithmetic::COMPONENT_ARITHMETIC_GATE;
use crate::model_arithmetic::NAMESPACE_ARITHMETIC_F64;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

const LHS: ArithmeticGateProperties = ArithmeticGateProperties::LHS;
const RHS: ArithmeticGateProperties = ArithmeticGateProperties::RHS;
const RESULT: ArithmeticGateProperties = ArithmeticGateProperties::RESULT;

const TYPE_NAME_ADD: &str = "add";

#[test]
fn behaviour_function_should_exist() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, TYPE_NAME_ADD);
    assert!(ARITHMETIC_GATES_F64.contains_key(&behaviour_ty));
    assert!(ARITHMETIC_GATES_F64.get(&behaviour_ty).is_some());
}

#[test]
fn arithmetic_gate_add_type_test() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, TYPE_NAME_ADD);
    let add_function = ARITHMETIC_GATES_F64.get(&behaviour_ty).expect("Failed to get function");
    let factory = ArithmeticGateF64Factory::new(behaviour_ty, add_function.clone());

    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, TYPE_NAME_ADD);

    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(LHS, json!(0.0))
        .property(RHS, json!(0.0))
        .property(RESULT, json!(0.0))
        .build();
    let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
    assert_eq!(BehaviourState::Connected, behaviour.get_state());
    assert_eq!(reactive_instance.id, behaviour.get_reactive_instance().id);
    // Validate
    behaviour.get_validator().validate().expect("Behaviour is invalid");
    // Disconnect
    behaviour.transition(BehaviourState::Ready).expect("Failed to disconnect");
    assert_eq!(BehaviourState::Ready, behaviour.get_state());
    // Connect
    behaviour.transition(BehaviourState::Connected).expect("Failed to connect");
    assert_eq!(BehaviourState::Connected, behaviour.get_state());
    // Reconnect
    behaviour
        .transition(BehaviourState::Ready)
        .and_then(|_| behaviour.transition(BehaviourState::Connected))
        .expect("Failed to connect");
    assert_eq!(BehaviourState::Connected, behaviour.get_state());

    // Set both inputs
    reactive_instance.set(LHS, json!(1.0));
    reactive_instance.set(RHS, json!(1.0));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(2.0, reactive_instance.as_f64(RESULT).unwrap());

    // Set lhs
    reactive_instance.set(LHS, json!(2.0));
    assert_eq!(3.0, reactive_instance.as_f64(RESULT).unwrap());

    // Set rhs
    reactive_instance.set(RHS, json!(2.0));
    assert_eq!(4.0, reactive_instance.as_f64(RESULT).unwrap());
}

#[test]
fn rx_add_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, TYPE_NAME_ADD);
    let reactive_instance = arithmetic_gate(&entity_ty);

    let rx_add = ArithmeticGateF64::from(reactive_instance.clone());

    {
        let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_ARITHMETIC_F64, TYPE_NAME_ADD);
        let not_function = ARITHMETIC_GATES_F64.get(&behaviour_ty).expect("Failed to get function");
        let not_factory = ArithmeticGateF64Factory::new(behaviour_ty, not_function.clone());
        let behaviour = not_factory.create(reactive_instance.clone()).expect("Failed to create behaviour");

        assert_eq!(NAMESPACE_ARITHMETIC_F64, behaviour.ty().namespace().as_str());
        assert_eq!(TYPE_NAME_ADD, behaviour.ty().type_name().as_str());

        rx_add.lhs(1.0);
        rx_add.rhs(1.0);
        assert_eq!(2.0, rx_add.result().unwrap());

        rx_add.lhs(2.0);
        assert_eq!(3.0, rx_add.result().unwrap());

        rx_add.rhs(2.0);
        assert_eq!(4.0, rx_add.result().unwrap());
    }
    // The behaviour has been dropped (no more changes)
    rx_add.lhs(0.0);
    rx_add.rhs(0.0);
    assert_eq!(4.0, rx_add.result().unwrap());
}

pub fn arithmetic_gate(entity_ty: &EntityTypeId) -> Arc<ReactiveEntityInstance> {
    ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(LHS, json!(0.0))
        .property(RHS, json!(0.0))
        .property(RESULT, json!(0.0))
        .component(COMPONENT_ARITHMETIC_GATE.clone())
        .build()
}
