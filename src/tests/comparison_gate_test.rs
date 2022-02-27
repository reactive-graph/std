use std::collections::HashMap;
use std::sync::Arc;

use serde_json::json;
use uuid::Uuid;

use crate::behaviour::entity::gate::ComparisonGate;
use crate::behaviour::entity::gate::ComparisonGateProperties;
use crate::behaviour::entity::gate::COMPARISON_GATES;
use crate::model::DataType;
use crate::model::EntityInstance;
use crate::model::EntityType;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::PropertyType;
use crate::model::ReactiveEntityInstance;
use crate::model::SocketType;
use crate::reactive::Gate;
use crate::reactive::Operation;

const LHS: ComparisonGateProperties = ComparisonGateProperties::LHS;
const RHS: ComparisonGateProperties = ComparisonGateProperties::RHS;
const RESULT: ComparisonGateProperties = ComparisonGateProperties::RESULT;

const COMPONENT_NAME_COMPARISON_GATE: &'static str = "comparison_gate";
const TYPE_NAME_EQUALS: &str = "equals";
const TYPE_NAME_GREATER_THAN_OR_EQUALS: &str = "greater_than_or_equals";

#[test]
fn behaviour_function_should_exist() {
    assert!(COMPARISON_GATES.contains_key(TYPE_NAME_EQUALS));
    assert!(COMPARISON_GATES.get(TYPE_NAME_EQUALS).is_some());
}

#[test]
fn equals_gate_test() {
    let property_types = vec![
        PropertyType::new_with_socket(LHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(RHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(RESULT, DataType::Number, SocketType::Output),
    ];
    let equals_type = EntityType::new(TYPE_NAME_EQUALS, "", "", vec![String::from(COMPONENT_NAME_COMPARISON_GATE)], property_types, Vec::new());
    let equals_function = COMPARISON_GATES.get(TYPE_NAME_EQUALS).unwrap();
    let mut properties = HashMap::new();
    properties.insert(LHS.into(), json!(LHS.default_value()));
    properties.insert(RHS.into(), json!(LHS.default_value()));
    properties.insert(RESULT.into(), json!(RESULT.default_value()));
    let equals_entity = EntityInstance::new(equals_type.name.clone(), Uuid::new_v4(), properties);
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

#[test]
fn greater_than_or_equals_gate_test() {
    let property_types = vec![
        PropertyType::new_with_socket(LHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(RHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(RESULT, DataType::Number, SocketType::Output),
    ];
    let gte_type = EntityType::new(
        TYPE_NAME_GREATER_THAN_OR_EQUALS,
        "",
        "",
        vec![String::from(COMPONENT_NAME_COMPARISON_GATE)],
        property_types,
        Vec::new(),
    );
    let gte_function = COMPARISON_GATES.get(TYPE_NAME_GREATER_THAN_OR_EQUALS).unwrap();
    let mut properties = HashMap::new();
    properties.insert(LHS.into(), json!(LHS.default_value()));
    properties.insert(RHS.into(), json!(LHS.default_value()));
    properties.insert(RESULT.into(), json!(RESULT.default_value()));
    let gte_entity = EntityInstance::new(gte_type.name.clone(), Uuid::new_v4(), properties);
    let gte_reactive_entity = Arc::new(ReactiveEntityInstance::from(gte_entity));
    let gte_behaviour = ComparisonGate::new(gte_reactive_entity.clone(), *gte_function);
    assert_eq!(TYPE_NAME_GREATER_THAN_OR_EQUALS, gte_behaviour.type_name().as_str());

    // === Reactive Entity API ===

    gte_reactive_entity.set(LHS, json!(5));
    gte_reactive_entity.set(RHS, json!(5));
    assert_eq!(true, gte_reactive_entity.as_bool(RESULT).unwrap());
    gte_reactive_entity.set(LHS, json!(6));
    assert_eq!(true, gte_reactive_entity.as_bool(RESULT).unwrap());
    gte_reactive_entity.set(RHS, json!(7));
    assert_eq!(false, gte_reactive_entity.as_bool(RESULT).unwrap());
    gte_reactive_entity.set(LHS, json!(7));
    assert_eq!(true, gte_reactive_entity.as_bool(RESULT).unwrap());

    // === Behaviour API ===

    gte_behaviour.lhs(json!(1));
    gte_behaviour.rhs(json!(2));
    assert_eq!(false, gte_behaviour.result().as_bool().unwrap());
    gte_behaviour.lhs(json!(2));
    assert_eq!(true, gte_behaviour.result().as_bool().unwrap());
    gte_behaviour.lhs(json!(3));
    assert_eq!(true, gte_behaviour.result().as_bool().unwrap());
    gte_behaviour.rhs(json!(4));
    assert_eq!(false, gte_behaviour.result().as_bool().unwrap());
}
