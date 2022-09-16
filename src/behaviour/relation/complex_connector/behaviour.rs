use crate::behaviour::relation::complex_connector::function::ComplexConnectorFunction;
use crate::behaviour::relation::connector::ConnectorProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveRelationInstance;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

pub struct ComplexConnector {
    /// The connector is a wrapper of a reactive relation instance.
    pub relation: Arc<ReactiveRelationInstance>,

    pub f: ComplexConnectorFunction,

    /// The handle id is the numeric representation (u128) of the UUID of the inbound property
    pub handle_id: u128,
}

impl ComplexConnector {
    pub fn from_relation<'a>(relation: Arc<ReactiveRelationInstance>, f: ComplexConnectorFunction) -> ComplexConnector {
        let mut connector = ComplexConnector { relation, f, handle_id: 0 };
        connector.connect();
        connector
    }

    pub fn connect(&mut self) {
        if let Some(outbound_property_name) = self.relation.as_string(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string()) {
            if let Some(inbound_property_name) = self.relation.as_string(ConnectorProperties::INBOUND_PROPERTY_NAME.to_string()) {
                if let Some(outbound_property) = self.relation.outbound.properties.get(&outbound_property_name) {
                    if let Some(_inbound_property) = self.relation.inbound.properties.get(&inbound_property_name) {
                        // let inbound = self.relation.inbound.clone();
                        let relation_instance = self.relation.clone();
                        // Create random handle id
                        self.handle_id = Uuid::new_v4().as_u128();
                        let f = self.f;
                        outbound_property.stream.read().unwrap().observe_with_handle(
                            move |value: &Value| {
                                f(value.clone(), inbound_property_name.clone(), relation_instance.clone());
                            },
                            self.handle_id,
                        );
                    }
                }
            }
        }
    }

    pub fn disconnect(&self) {
        if let Some(outbound_property_name) = self.relation.as_string(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string()) {
            if let Some(outbound_property) = self.relation.outbound.properties.get(&outbound_property_name) {
                outbound_property.stream.read().unwrap().remove(self.handle_id);
            }
        }
    }

    pub fn type_name<S: Into<String>>(type_name: S, outbound_property_name: S, inbound_property_name: S) -> String {
        format!("{}--{}--{}", type_name.into(), outbound_property_name.into(), inbound_property_name.into())
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ComplexConnector {
    fn drop(&mut self) {
        self.disconnect();
    }
}
