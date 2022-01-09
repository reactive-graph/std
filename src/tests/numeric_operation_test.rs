use std::f64::consts::PI;

use serde_json::json;

use crate::model::{DataType, EntityInstance, EntityType, PropertyInstanceGetter, PropertyInstanceSetter, PropertyType, ReactiveEntityInstance, SocketType};

use crate::behaviour::entity::operation::function::NUMERIC_OPERATIONS;
use crate::behaviour::entity::operation::properties::NumericOperationProperties;
use crate::behaviour::entity::operation::NumericOperation;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

const LHS: NumericOperationProperties = NumericOperationProperties::LHS;
const RESULT: NumericOperationProperties = NumericOperationProperties::RESULT;

const COMPONENT_NAME_NUMERIC_OPERATION: &'static str = "numeric_operation";
const TYPE_NAME_SIN: &'static str = "sin";

#[test]
fn behaviour_function_should_exist() {
    assert!(NUMERIC_OPERATIONS.contains_key(TYPE_NAME_SIN));
    assert!(NUMERIC_OPERATIONS.get(TYPE_NAME_SIN).is_some());
}

#[test]
fn numeric_operation_sin_type_test() {
    let property_types = vec![
        PropertyType::new_with_socket(NumericOperationProperties::LHS, DataType::Number, SocketType::Input),
        PropertyType::new_with_socket(NumericOperationProperties::RESULT, DataType::Number, SocketType::Output),
    ];
    let sin_type = EntityType::new(
        TYPE_NAME_SIN,
        "",
        vec![String::from(COMPONENT_NAME_NUMERIC_OPERATION)],
        Vec::new(),
        property_types,
        Vec::new(),
    );
    let sin_function = NUMERIC_OPERATIONS.get(TYPE_NAME_SIN).unwrap();
    let mut properties = HashMap::new();
    properties.insert(NumericOperationProperties::LHS.into(), json!(NumericOperationProperties::LHS.default_value()));
    properties.insert(NumericOperationProperties::RESULT.into(), json!(NumericOperationProperties::RESULT.default_value()));
    let sin_entity = EntityInstance::new(sin_type.name.clone(), Uuid::new_v4(), properties);
    let sin_reactive_entity = Arc::new(ReactiveEntityInstance::from(sin_entity));
    let sin_behaviour = NumericOperation::new(sin_reactive_entity.clone(), *sin_function);
    assert_eq!(TYPE_NAME_SIN, sin_behaviour.type_name().as_str());

    // Set the input value
    sin_reactive_entity.set(NumericOperationProperties::LHS, json!(0.0));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(0.0, sin_reactive_entity.as_f64(NumericOperationProperties::RESULT).unwrap());

    // Set the input value
    sin_reactive_entity.set(NumericOperationProperties::LHS, json!(PI / 2.0));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(1.0, sin_reactive_entity.as_f64(NumericOperationProperties::RESULT).unwrap());
}
