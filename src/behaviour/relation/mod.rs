use std::collections::HashMap;

use serde_json::json;
use serde_json::Value;

use crate::behaviour::relation::properties::ConnectorProperties;

pub mod complex_connector;
pub mod connector;
pub mod properties;
pub mod relation_behaviour_provider;

/// The relation instance of type connector contains exactly two properties
/// which contains the names of the entity properties.
pub fn get_connector_relation_properties(outbound_property_name: String, inbound_property_name: String) -> HashMap<String, Value> {
    let mut properties = HashMap::new();
    properties.insert(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string(), json!(outbound_property_name));
    properties.insert(ConnectorProperties::INBOUND_PROPERTY_NAME.to_string(), json!(inbound_property_name));
    properties
}
