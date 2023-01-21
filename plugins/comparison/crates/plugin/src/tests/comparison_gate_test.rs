use inexor_rgf_model_comparison::ComparisonGate;
use inexor_rgf_model_result::ResultBoolean;
use serde_json::json;

use crate::behaviour::entity::gate::behaviour::ComparisonGateFactory;
use crate::behaviour::entity::gate::COMPARISON_GATES;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::BehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model_comparison::ComparisonGateProperties;
use crate::model_comparison::NAMESPACE_COMPARISON;
use crate::model_result::ResultBooleanProperties;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

const LHS: ComparisonGateProperties = ComparisonGateProperties::LHS;
const RHS: ComparisonGateProperties = ComparisonGateProperties::RHS;
const RESULT: ResultBooleanProperties = ResultBooleanProperties::RESULT;

const TYPE_NAME_EQUALS: &str = "equals";
const TYPE_NAME_GREATER_THAN_OR_EQUALS: &str = "greater_than_or_equals";

#[test]
fn behaviour_function_should_exist() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_COMPARISON, TYPE_NAME_EQUALS);
    assert!(COMPARISON_GATES.contains_key(&behaviour_ty));
    assert!(COMPARISON_GATES.get(&behaviour_ty).is_some());
}

#[test]
fn equals_gate_test() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_COMPARISON, TYPE_NAME_EQUALS);
    let equals_function = COMPARISON_GATES.get(&behaviour_ty).expect("Failed to get function");
    let factory = ComparisonGateFactory::new(behaviour_ty, equals_function.clone());

    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_COMPARISON, TYPE_NAME_EQUALS);

    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(LHS, json!(false))
        .property(RHS, json!(false))
        .property(RESULT, json!(true))
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
    reactive_instance.set(LHS, json!(true));
    reactive_instance.set(RHS, json!(true));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());

    let mut lhs = 0.0;
    let mut rhs = 0.0;
    reactive_instance.set(LHS, json!(lhs));
    reactive_instance.set(RHS, json!(rhs));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    lhs = 1.0;
    reactive_instance.set(LHS, json!(lhs));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());
    rhs = 1.0;
    reactive_instance.set(RHS, json!(rhs));
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    lhs = 0.0;
    reactive_instance.set(LHS, json!(lhs));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());

    let mut lhs: i64 = 0;
    let mut rhs: i64 = 0;
    reactive_instance.set(LHS, json!(lhs));
    reactive_instance.set(RHS, json!(rhs));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    lhs = 1;
    reactive_instance.set(LHS, json!(lhs));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());
    rhs = 1;
    reactive_instance.set(RHS, json!(rhs));
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    lhs = 0;
    reactive_instance.set(LHS, json!(lhs));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());

    let mut lhs: u64 = 0;
    let mut rhs: u64 = 0;
    reactive_instance.set(LHS, json!(lhs));
    reactive_instance.set(RHS, json!(rhs));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    lhs = 1;
    reactive_instance.set(LHS, json!(lhs));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());
    rhs = 1;
    reactive_instance.set(RHS, json!(rhs));
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    lhs = 0;
    reactive_instance.set(LHS, json!(lhs));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());
}

#[test]
fn greater_than_or_equals_gate_test() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_COMPARISON, TYPE_NAME_GREATER_THAN_OR_EQUALS);
    let greater_than_or_equals_function = COMPARISON_GATES.get(&behaviour_ty).expect("Failed to get function");
    let factory = ComparisonGateFactory::new(behaviour_ty, greater_than_or_equals_function.clone());

    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_COMPARISON, TYPE_NAME_GREATER_THAN_OR_EQUALS);

    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(LHS, json!(0.0))
        .property(RHS, json!(0.0))
        .property(RESULT, json!(true))
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

    let mut lhs = 0.0;
    let mut rhs = 0.0;
    reactive_instance.set(LHS, json!(lhs));
    reactive_instance.set(RHS, json!(rhs));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    lhs = 1.0;
    reactive_instance.set(LHS, json!(lhs));
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    rhs = 1.0;
    reactive_instance.set(RHS, json!(rhs));
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    lhs = 0.0;
    reactive_instance.set(LHS, json!(lhs));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());

    let mut lhs: i64 = 0;
    let mut rhs: i64 = 0;
    reactive_instance.set(LHS, json!(lhs));
    reactive_instance.set(RHS, json!(rhs));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    lhs = 1;
    reactive_instance.set(LHS, json!(lhs));
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    rhs = 1;
    reactive_instance.set(RHS, json!(rhs));
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    lhs = 0;
    reactive_instance.set(LHS, json!(lhs));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());

    let mut lhs: u64 = 0;
    let mut rhs: u64 = 0;
    reactive_instance.set(LHS, json!(lhs));
    reactive_instance.set(RHS, json!(rhs));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    lhs = 1;
    reactive_instance.set(LHS, json!(lhs));
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    rhs = 1;
    reactive_instance.set(RHS, json!(rhs));
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    lhs = 0;
    reactive_instance.set(LHS, json!(lhs));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());
}

#[test]
fn rx_equals_gate_test() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_COMPARISON, TYPE_NAME_EQUALS);
    let equals_function = COMPARISON_GATES.get(&behaviour_ty).expect("Failed to get function");
    let factory = ComparisonGateFactory::new(behaviour_ty, equals_function.clone());

    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_COMPARISON, TYPE_NAME_EQUALS);

    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(LHS, json!(false))
        .property(RHS, json!(false))
        .property(RESULT, json!(true))
        .build();

    let rx_equals = ComparisonGate::from(reactive_instance.clone());

    {
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());
        assert_eq!(reactive_instance.id, behaviour.get_reactive_instance().id);

        rx_equals.lhs(json!("a"));
        rx_equals.rhs(json!("b"));
        assert_eq!(false, rx_equals.result().unwrap());
        rx_equals.rhs(json!("a"));
        assert_eq!(true, rx_equals.result().unwrap());
        rx_equals.lhs(json!("b"));
        assert_eq!(false, rx_equals.result().unwrap());
        rx_equals.lhs(json!(1));
        assert_eq!(false, rx_equals.result().unwrap());
        rx_equals.rhs(json!(1));
        assert_eq!(true, rx_equals.result().unwrap());
    }
}
