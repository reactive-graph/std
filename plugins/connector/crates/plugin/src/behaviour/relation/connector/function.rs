use std::clone::Clone;
use std::sync::Arc;
use std::sync::LazyLock;

use log::debug;
use log::trace;
use reactive_graph_behaviour_model_impl::relation::RelationBehaviourFactoryCreator;
use reactive_graph_behaviour_model_impl::relation::function::RelationBehaviourFunctions;
use reactive_graph_behaviour_model_impl::relation::function::RelationBehaviourFunctionsStorage;
use serde_json::Value;
use serde_json::json;

use reactive_graph_std_connector_model::*;

use crate::behaviour::relation::connector::ConnectorFactory;

pub type ConnectorFunction = fn(&Value) -> Value;

/// This connector logs the value before propagation (log level debug)
pub const FN_DEBUG_CONNECTOR: ConnectorFunction = |v| {
    debug!("connector propagates value {}", v.to_string());
    v.clone()
};

/// This is the default connector type, which simply does nothing than propagate the value
pub const FN_DEFAULT_CONNECTOR: ConnectorFunction = |v| v.clone();

/// This connector parses a string value and propagates a float value
pub const FN_PARSE_FLOAT_CONNECTOR: ConnectorFunction = |v| {
    if v.is_f64() {
        return v.clone();
    }
    // json!(v.as_str().map(str::parse::<f64>).unwrap_or(0.0f64))
    let str_value = v.as_str();
    if str_value.is_none() {
        return json!(0.0);
    }
    str_value
        .unwrap()
        .parse::<f64>()
        .map(|int_value| json!(int_value))
        .unwrap_or_else(|_| json!(0.0))
};

/// This connector parses a string value and propagates a int value
pub const FN_PARSE_INT_CONNECTOR: ConnectorFunction = |v| {
    if v.is_i64() {
        return v.clone();
    }
    // json!(v.as_str().map(str::parse::<i64>).unwrap_or(0i64))
    let str_value = v.as_str();
    if str_value.is_none() {
        return json!(0);
    }
    str_value.unwrap().parse::<i64>().map(|int_value| json!(int_value)).unwrap_or_else(|_| json!(0))
};

/// This connector converts the value of any type to string before propagation
pub const FN_TO_STRING_CONNECTOR: ConnectorFunction = |v| json!(v.to_string());

/// This connector logs the value before propagation (log level trace)
pub const FN_TRACE_CONNECTOR: ConnectorFunction = |v| {
    trace!("connector propagates value {}", v.to_string());
    v.clone()
};

const FACTORY_CREATOR: RelationBehaviourFactoryCreator<ConnectorFunction> = |ty, f| Arc::new(ConnectorFactory::new(ty.clone(), f));

pub static CONNECTOR_BEHAVIOURS: RelationBehaviourFunctionsStorage<ConnectorFunction> = LazyLock::new(|| {
    RelationBehaviourFunctions::<ConnectorFunction>::with_namespace(NAMESPACE_CONNECTOR, FACTORY_CREATOR)
        .behaviour("debug_connector", FN_DEBUG_CONNECTOR)
        .behaviour("default_connector", FN_DEFAULT_CONNECTOR)
        .behaviour("parse_float_connector", FN_PARSE_FLOAT_CONNECTOR)
        .behaviour("parse_int_connector", FN_PARSE_INT_CONNECTOR)
        .behaviour("to_string_connector", FN_TO_STRING_CONNECTOR)
        .behaviour("trace_connector", FN_TRACE_CONNECTOR)
        .get()
});
