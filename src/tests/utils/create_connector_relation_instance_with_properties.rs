use std::collections::HashMap;
use std::sync::Arc;

use serde_json::json;

use crate::behaviour::relation::properties::ConnectorProperties;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveRelationInstance;

pub fn create_connector_relation_instance_with_properties<S: Into<String>>(
    outbound_entity: Arc<ReactiveEntityInstance>,
    type_name: S,
    inbound_entity: Arc<ReactiveEntityInstance>,
    outbound_property_name: S,
    inbound_property_name: S,
) -> ReactiveRelationInstance {
    let mut properties = HashMap::new();
    properties.insert(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string(), json!(outbound_property_name.into()));
    properties.insert(ConnectorProperties::INBOUND_PROPERTY_NAME.to_string(), json!(inbound_property_name.into()));
    ReactiveRelationInstance::create_with_properties(outbound_entity.clone(), type_name.into().clone(), inbound_entity.clone(), properties)
}
