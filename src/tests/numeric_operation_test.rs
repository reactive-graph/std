use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::Arc;

use serde_json::json;
use uuid::Uuid;

use crate::behaviour::entity::operation::function::NUMERIC_OPERATIONS;
use crate::behaviour::entity::operation::properties::NumericOperationProperties;
use crate::behaviour::entity::operation::NumericOperation;
use crate::model::DataType;
use crate::model::EntityInstance;
use crate::model::EntityType;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::PropertyType;
use crate::model::ReactiveEntityInstance;
use crate::model::SocketType;

const NAMESPACE: &str = "test";
const COMPONENT_NAME_NUMERIC_OPERATION: &str = "numeric_operation";
const TYPE_NAME_SIN: &str = "sin";

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
    let sin_type = EntityType::new(NAMESPACE, TYPE_NAME_SIN, "", vec![String::from(COMPONENT_NAME_NUMERIC_OPERATION)], property_types, Vec::new());
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
