use std::sync::Arc;
use std::sync::LazyLock;
use std::thread;
use std::time::Duration;

use reactive_graph_behaviour_model_impl::relation::RelationBehaviourFactoryCreator;
use reactive_graph_behaviour_model_impl::relation::function::RelationBehaviourFunctions;
use reactive_graph_behaviour_model_impl::relation::function::RelationBehaviourFunctionsStorage;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use serde_json::Value;
use serde_json::json;

use reactive_graph_std_connector_model::BufferProperties::BUFFER;
use reactive_graph_std_connector_model::BufferProperties::BUFFER_SIZE;
use reactive_graph_std_connector_model::NAMESPACE_CONNECTOR;

use crate::behaviour::relation::complex_connector::ComplexConnectorFactory;

pub type ComplexConnectorFunction = fn(Value, String, ReactiveRelation);

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

const FACTORY_CREATOR: RelationBehaviourFactoryCreator<ComplexConnectorFunction> = |ty, f| Arc::new(ComplexConnectorFactory::new(ty.clone(), f));

pub static COMPLEX_CONNECTOR_BEHAVIOURS: RelationBehaviourFunctionsStorage<ComplexConnectorFunction> = LazyLock::new(|| {
    RelationBehaviourFunctions::<ComplexConnectorFunction>::with_namespace(NAMESPACE_CONNECTOR, FACTORY_CREATOR)
        .behaviour("delay_connector", FN_DELAY_CONNECTOR)
        .behaviour("debounce_connector", FN_DEBOUNCE_CONNECTOR)
        .behaviour("buffered_fifo_connector", FN_BUFFERED_FIFO_CONNECTOR)
        .behaviour("numeric_interpolation_connector", FN_NUMERIC_INTERPOLATION_CONNECTOR)
        .behaviour("threaded_connector", FN_THREADED_CONNECTOR)
        .behaviour("increment_by_connector", FN_INCREMENT_BY_CONNECTOR)
        .behaviour("decrement_by_connector", FN_DECREMENT_BY_CONNECTOR)
        .get()
});
