use std::sync::Arc;

use serde_json::{json, Value};

use crate::behaviour::entity::gate::ComparisonGate;
use crate::behaviour::entity::gate::ComparisonGateProperties;
use crate::behaviour::entity::gate::COMPARISON_GATES;
use crate::model::{
    DataType, EntityInstance, EntityType, PropertyType, ReactiveEntityInstance, SocketType,
};
use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
use inexor_rgf_core_reactive::{Gate, Operation};
use std::collections::HashMap;
use uuid::Uuid;

const LHS: ComparisonGateProperties = ComparisonGateProperties::LHS;
const RHS: ComparisonGateProperties = ComparisonGateProperties::RHS;
const RESULT: ComparisonGateProperties = ComparisonGateProperties::RESULT;

const COMPONENT_NAME_COMPARISON_GATE: &'static str = "comparison_gate";
const TYPE_NAME_EQUALS: &str = "equals";

#[test]
fn behaviour_function_should_exist() {
    assert!(COMPARISON_GATES.contains_key(TYPE_NAME_EQUALS));
    assert!(COMPARISON_GATES.get(TYPE_NAME_EQUALS).is_some());
}

#[test]
fn and_gate_test() {
    let property_types = vec![
        PropertyType::new_with_socket(LHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(RHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(RESULT, DataType::Number, SocketType::Output),
    ];
    let and_type = EntityType::new(
        TYPE_NAME_EQUALS,
        "",
        vec![String::from(COMPONENT_NAME_COMPARISON_GATE)],
        Vec::new(),
        property_types,
        Vec::new(),
    );
    let equals_function = COMPARISON_GATES.get(TYPE_NAME_EQUALS).unwrap();
    let mut properties = HashMap::new();
    properties.insert(LHS.into(), json!(LHS.default_value()));
    properties.insert(RHS.into(), json!(LHS.default_value()));
    properties.insert(RESULT.into(), json!(RESULT.default_value()));
    let equals_entity = EntityInstance::new(and_type.name.clone(), Uuid::new_v4(), properties);
    let equals_reactive_entity = Arc::new(ReactiveEntityInstance::from(equals_entity));
    let equals_behaviour = ComparisonGate::new(equals_reactive_entity.clone(), *equals_function);
    assert_eq!(TYPE_NAME_EQUALS, equals_behaviour.type_name().as_str());

    // === Reactive Entity API ===

    equals_reactive_entity.set(LHS, json!(5));
    equals_reactive_entity.set(RHS, json!(5));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(true, equals_reactive_entity.as_bool(RESULT).unwrap());
    equals_reactive_entity.set(LHS, json!(6));
    assert_eq!(false, equals_reactive_entity.as_bool(RESULT).unwrap());
    equals_reactive_entity.set(RHS, json!(7));
    assert_eq!(false, equals_reactive_entity.as_bool(RESULT).unwrap());
    equals_reactive_entity.set(LHS, json!(6));
    assert_eq!(false, equals_reactive_entity.as_bool(RESULT).unwrap());

    // === Behaviour API ===

    equals_behaviour.lhs(json!(1));
    equals_behaviour.rhs(json!(2));
    assert_eq!(false, equals_behaviour.result().as_bool().unwrap());
    equals_behaviour.lhs(json!(2));
    assert_eq!(true, equals_behaviour.result().as_bool().unwrap());
    equals_behaviour.rhs(json!(3));
    assert_eq!(false, equals_behaviour.result().as_bool().unwrap());
    equals_behaviour.lhs(json!(3));
    assert_eq!(true, equals_behaviour.result().as_bool().unwrap());
}
