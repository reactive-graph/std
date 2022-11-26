use std::sync::Arc;
use std::thread;
use std::time::Duration;

use serde_json::json;
use serde_json::Value;

use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveRelationInstance;
use crate::model_connector::BufferProperties::BUFFER;
use crate::model_connector::BufferProperties::BUFFER_SIZE;
use crate::model_connector::NAMESPACE_CONNECTOR;
use crate::reactive::behaviour_functions;

pub type ComplexConnectorFunction = fn(Value, String, Arc<ReactiveRelationInstance>);

pub const FN_DELAY_CONNECTOR: ComplexConnectorFunction = |new_value, inbound_property_name, relation_instance| {
    let delay_in_millis = relation_instance.get("delay").and_then(|v| v.as_u64()).unwrap_or(10);
    thread::sleep(Duration::from_millis(delay_in_millis));
    relation_instance.inbound.set(inbound_property_name, new_value);
};

pub const FN_DEBOUNCE_CONNECTOR: ComplexConnectorFunction = |new_value, inbound_property_name, relation_instance| {
    if let Some(old_value) = relation_instance.inbound.get(inbound_property_name.clone()) {
        if old_value != new_value {
            // Only update inbound if inbound value not equals the outbound value
            relation_instance.inbound.set(inbound_property_name, new_value);
        }
    }
};

pub const FN_BUFFERED_FIFO_CONNECTOR: ComplexConnectorFunction = |new_value, inbound_property_name, relation_instance| {
    let buffer_size = relation_instance
        .get(BUFFER_SIZE)
        .and_then(|v| v.as_u64())
        .and_then(|v| usize::try_from(v).ok())
        .unwrap_or(10);
    let mut buffer = relation_instance.get(BUFFER).and_then(|v| v.as_array().cloned()).unwrap_or_default();
    buffer.insert(0, new_value);
    // Only update inbound if FIFO is full
    while buffer.len() > buffer_size {
        if let Some(v) = buffer.pop() {
            relation_instance.inbound.set(&inbound_property_name, v);
        }
    }
    relation_instance.set(BUFFER, json!(buffer));
};

pub const FN_NUMERIC_INTERPOLATION_CONNECTOR: ComplexConnectorFunction = |new_value, inbound_property_name, relation_instance| {
    let buffer_size = relation_instance
        .get(BUFFER_SIZE)
        .and_then(|v| v.as_u64())
        .and_then(|v| usize::try_from(v).ok())
        .unwrap_or(10);
    let mut buffer = relation_instance.get(BUFFER).and_then(|v| v.as_array().cloned()).unwrap_or_default();
    buffer.insert(0, new_value);
    while buffer.len() > buffer_size {
        buffer.pop();
    }
    relation_instance.set(BUFFER, json!(buffer));
    // Calculate average
    let average = buffer.iter().filter_map(|v| v.as_f64()).sum::<f64>() / buffer.len() as f64;
    relation_instance.inbound.set(&inbound_property_name, json!(average));
};

pub const FN_THREADED_CONNECTOR: ComplexConnectorFunction = |new_value, inbound_property_name, relation_instance| {
    let relation_instance_2 = relation_instance.clone();
    thread::spawn(move || {
        relation_instance_2.inbound.set(inbound_property_name, new_value);
    });
};

pub const FN_INCREMENT_BY_CONNECTOR: ComplexConnectorFunction = |increment_by, inbound_property_name, relation_instance| {
    if let Some(old_value) = relation_instance.inbound.get(&inbound_property_name).and_then(|v| v.as_i64()) {
        if let Some(increment_by) = increment_by.as_i64() {
            let new_value = old_value + increment_by;
            relation_instance.inbound.set(&inbound_property_name, json!(new_value));
        }
    }
};

pub const FN_DECREMENT_BY_CONNECTOR: ComplexConnectorFunction = |decrement_by, inbound_property_name, relation_instance| {
    if let Some(old_value) = relation_instance.inbound.get(&inbound_property_name).and_then(|v| v.as_i64()) {
        if let Some(decrement_by) = decrement_by.as_i64() {
            let new_value = old_value - decrement_by;
            relation_instance.inbound.set(&inbound_property_name, json!(new_value));
        }
    }
};

behaviour_functions!(
    COMPLEX_CONNECTOR_BEHAVIOURS,
    ComplexConnectorFunction,
    NAMESPACE_CONNECTOR,
    "delay_connector",
    FN_DELAY_CONNECTOR,
    "debounce_connector",
    FN_DEBOUNCE_CONNECTOR,
    "buffered_fifo_connector",
    FN_BUFFERED_FIFO_CONNECTOR,
    "numeric_interpolation_connector",
    FN_NUMERIC_INTERPOLATION_CONNECTOR,
    "threaded_connector",
    FN_THREADED_CONNECTOR,
    "increment_by_connector",
    FN_INCREMENT_BY_CONNECTOR,
    "decrement_by_connector",
    FN_DECREMENT_BY_CONNECTOR
);
