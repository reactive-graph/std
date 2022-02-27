use crate::behaviour::relation::connector::properties::ConnectorProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveRelationInstance;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub static TYPE_NAME_CONNECTOR: &str = "connector";

pub type ConnectorFunction = fn(Value) -> Value;

/// A connector connects a property of the outbound entity instance with
/// a property of the inbound entity instance.
///
/// In theory it's also possible to connect two properties of the same entity instance.
///
/// On construction the streams are connected. No type checks are performed.
///
/// On destruction of the connector, the stream will be removed.
///
/// TODO: This behaviour doesn't handle changes of the relation properties outbound_property_name or the inbound_property_name (rewiring / reconstruction)
pub struct Connector {
    /// The connector is a wrapper of a reactive relation instance.
    pub relation: Arc<ReactiveRelationInstance>,

    pub f: ConnectorFunction,

    /// The handle id is the numeric representation (u128) of the UUID of the inbound property
    pub handle_id: u128,
}

impl Connector {
    pub fn from_relation<'a>(
        relation: Arc<ReactiveRelationInstance>,
        f: ConnectorFunction,
    ) -> Connector {
        let mut connector = Connector {
            relation,
            f,
            handle_id: 0,
        };
        connector.connect();
        connector
    }

    /// Constructs a new connector using an outbound entity (+ name of the property) and
    /// an inbound entity (+ name of the property)
    ///
    /// TODO: This doesn't handle the different types of connectors yet
    pub fn new(
        outbound: Arc<ReactiveEntityInstance>,
        outbound_property_name: String,
        inbound: Arc<ReactiveEntityInstance>,
        inbound_property_name: String,
    ) -> Connector {
        let properties =
            get_connector_relation_properties(outbound_property_name, inbound_property_name);
        let relation = Arc::new(ReactiveRelationInstance::create_with_properties(
            outbound,
            TYPE_NAME_CONNECTOR.to_string(),
            inbound,
            properties,
        ));
        Connector::from_relation(relation, |v| v)
    }

    pub fn connect(&mut self) {
        if let Some(outbound_property_name) = self
            .relation
            .as_string(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string())
        {
            if let Some(inbound_property_name) = self
                .relation
                .as_string(ConnectorProperties::INBOUND_PROPERTY_NAME.to_string())
            {
                if let Some(outbound_property) = self
                    .relation
                    .outbound
                    .properties
                    .get(&outbound_property_name)
                {
                    if let Some(_inbound_property) =
                        self.relation.inbound.properties.get(&inbound_property_name)
                    {
                        let inbound = self.relation.inbound.clone();
                        // Create random handle id
                        self.handle_id = Uuid::new_v4().as_u128();
                        let f = self.f;
                        outbound_property
                            .stream
                            .read()
                            .unwrap()
                            .observe_with_handle(
                                move |value: &Value| {
                                    inbound.set(inbound_property_name.clone(), f(value.clone()));
                                },
                                self.handle_id,
                            );
                    }
                }
            }
        }
    }

    pub fn disconnect(&self) {
        if let Some(outbound_property_name) = self
            .relation
            .as_string(ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string())
        {
            if let Some(outbound_property) = self
                .relation
                .outbound
                .properties
                .get(&outbound_property_name)
            {
                outbound_property
                    .stream
                    .read()
                    .unwrap()
                    .remove(self.handle_id);
            }
        }
    }

    pub fn type_name<S: Into<String>>(
        type_name: S,
        outbound_property_name: S,
        inbound_property_name: S,
    ) -> String {
        format!(
            "{}--{}--{}",
            type_name.into(),
            outbound_property_name.into(),
            inbound_property_name.into()
        )
    }
}

/// Automatically disconnect streams on destruction
impl Drop for Connector {
    fn drop(&mut self) {
        self.disconnect();
    }
}

/// The relation instance of type connector contains exactly two properties
/// which contains the names of the entity properties.
fn get_connector_relation_properties(
    outbound_property_name: String,
    inbound_property_name: String,
) -> HashMap<String, Value> {
    let mut properties = HashMap::new();
    properties.insert(
        ConnectorProperties::OUTBOUND_PROPERTY_NAME.to_string(),
        json!(outbound_property_name),
    );
    properties.insert(
        ConnectorProperties::INBOUND_PROPERTY_NAME.to_string(),
        json!(inbound_property_name),
    );
    properties
}
