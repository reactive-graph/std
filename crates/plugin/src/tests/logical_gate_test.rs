use serde_json::json;

use crate::behaviour::entity::gate::LogicalGateFactory;
use crate::behaviour::entity::gate::LOGICAL_GATES;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::BehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model_logical::LogicalGateProperties;
use crate::model_logical::COMPONENT_LOGICAL_GATE;
use crate::model_logical::NAMESPACE_LOGICAL;
use crate::reactive::BehaviourFactory;

const LHS: LogicalGateProperties = LogicalGateProperties::LHS;
const RHS: LogicalGateProperties = LogicalGateProperties::RHS;
const RESULT: LogicalGateProperties = LogicalGateProperties::RESULT;

const TYPE_NAME_AND: &str = "and";

#[test]
fn logical_gate_behaviour_function_should_exist() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_AND);
    assert!(LOGICAL_GATES.contains_key(&behaviour_ty));
    assert!(LOGICAL_GATES.get(&behaviour_ty).is_some());
}

#[test]
fn and_gate_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_AND);
    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(LHS, json!(false))
        .property(RHS, json!(false))
        .property(RESULT, json!(false))
        .component(COMPONENT_LOGICAL_GATE.clone())
        .build();

    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_AND);
    let and_function = LOGICAL_GATES.get(&behaviour_ty).expect("Failed to get function");
    let and_factory = LogicalGateFactory::new(behaviour_ty, and_function.clone());
    let behaviour = and_factory.create(reactive_instance.clone()).expect("Failed to create behaviour");

    assert_eq!(NAMESPACE_LOGICAL, behaviour.ty().namespace().as_str());
    assert_eq!(TYPE_NAME_AND, behaviour.ty().type_name().as_str());

    // === Reactive Entity API ===

    reactive_instance.set(LHS, json!(true));
    reactive_instance.set(RHS, json!(true));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    reactive_instance.set(LHS, json!(false));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());
    reactive_instance.set(RHS, json!(false));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());
    reactive_instance.set(LHS, json!(true));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());
    reactive_instance.set(RHS, json!(true));
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
}

#[test]
fn incomplete_and_gate_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_AND);
    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .component(COMPONENT_LOGICAL_GATE.clone())
        .build();

    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_AND);
    let and_function = LOGICAL_GATES.get(&behaviour_ty).expect("Failed to get function");
    let and_factory = LogicalGateFactory::new(behaviour_ty, and_function.clone());
    let behaviour = and_factory.create(reactive_instance.clone());
    assert!(behaviour.is_err());
}
