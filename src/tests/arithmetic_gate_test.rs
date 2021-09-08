use std::f64::consts::PI;

use serde_json::json;

use crate::model::{
    DataType, EntityInstance, EntityType, PropertyInstanceGetter, PropertyInstanceSetter,
    PropertyType, ReactiveEntityInstance, SocketType,
};

use crate::behaviour::entity::gate::function::ARITHMETIC_GATES;
use crate::behaviour::entity::gate::properties::ArithmeticGateProperties;
use crate::behaviour::entity::gate::ArithmeticGate;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

const LHS: ArithmeticGateProperties = ArithmeticGateProperties::LHS;
const RHS: ArithmeticGateProperties = ArithmeticGateProperties::RHS;
const RESULT: ArithmeticGateProperties = ArithmeticGateProperties::RESULT;

const COMPONENT_NAME_ARITHMETIC_GATE: &'static str = "arithmetic_gate";
const TYPE_NAME_ADD: &'static str = "add";

#[test]
fn behaviour_function_should_exist() {
    assert!(ARITHMETIC_GATES.contains_key(TYPE_NAME_ADD));
    assert!(ARITHMETIC_GATES.get(TYPE_NAME_ADD).is_some());
}

#[test]
fn arithmetic_gate_add_type_test() {
    let property_types = vec![
        PropertyType::new_with_socket(LHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(RHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(RESULT, DataType::Number, SocketType::Output),
    ];
    let add_type = EntityType::new(
        TYPE_NAME_ADD,
        "",
        vec![String::from(COMPONENT_NAME_ARITHMETIC_GATE)],
        Vec::new(),
        property_types,
        Vec::new(),
    );
    let add_function = ARITHMETIC_GATES.get(TYPE_NAME_ADD).unwrap();
    let mut properties = HashMap::new();
    properties.insert(LHS.into(), json!(LHS.default_value()));
    properties.insert(RHS.into(), json!(LHS.default_value()));
    properties.insert(RESULT.into(), json!(RESULT.default_value()));
    let add_entity = EntityInstance::new(add_type.name.clone(), Uuid::new_v4(), properties);
    let add_reactive_entity = Arc::new(ReactiveEntityInstance::from(add_entity));
    let add_behaviour = ArithmeticGate::new(add_reactive_entity.clone(), *add_function);
    assert_eq!(TYPE_NAME_ADD, add_behaviour.type_name().as_str());

    // Set both inputs
    add_reactive_entity.set(LHS, json!(1.0));
    add_reactive_entity.set(RHS, json!(1.0));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(2.0, add_reactive_entity.as_f64(RESULT).unwrap());

    // Set lhs
    add_reactive_entity.set(LHS, json!(2.0));
    assert_eq!(3.0, add_reactive_entity.as_f64(RESULT).unwrap());

    // Set rhs
    add_reactive_entity.set(RHS, json!(2.0));
    assert_eq!(4.0, add_reactive_entity.as_f64(RESULT).unwrap());
}
