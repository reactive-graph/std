use log::debug;
use log::trace;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

pub type ConnectorFunction = fn(Value) -> Value;

/// This connector logs the value before propagation (log level debug)
pub const FN_DEBUG_CONNECTOR: ConnectorFunction = |v| {
    debug!("connector propagates value {}", v.to_string());
    v
};

/// This is the default connector type, which simply does nothing than propagate the value
pub const FN_DEFAULT_CONNECTOR: ConnectorFunction = |v| v;

/// This connector parses a string value and propagates a float value
pub const FN_PARSE_FLOAT_CONNECTOR: ConnectorFunction = |v| {
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
    v
};

lazy_static! {
    pub static ref CONNECTORS: HashMap<&'static str, ConnectorFunction> = vec![
        ("debug_connector", FN_DEBUG_CONNECTOR),
        ("default_connector", FN_DEFAULT_CONNECTOR),
        ("parse_float_connector", FN_PARSE_FLOAT_CONNECTOR),
        ("parse_int_connector", FN_PARSE_INT_CONNECTOR),
        ("to_string_connector", FN_TO_STRING_CONNECTOR),
        ("trace_connector", FN_TRACE_CONNECTOR),
    ]
    .into_iter()
    .collect();
}
