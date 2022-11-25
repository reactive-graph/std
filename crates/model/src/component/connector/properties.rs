use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::AsRefStr;
use strum_macros::Display;
use strum_macros::IntoStaticStr;

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ConnectorProperties {
    #[strum(serialize = "outbound_property_name")]
    OUTBOUND_PROPERTY_NAME,
    #[strum(serialize = "inbound_property_name")]
    INBOUND_PROPERTY_NAME,
}

impl ConnectorProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ConnectorProperties::OUTBOUND_PROPERTY_NAME => json!(""),
            ConnectorProperties::INBOUND_PROPERTY_NAME => json!(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ConnectorProperties::OUTBOUND_PROPERTY_NAME),
            NamedProperty::from(ConnectorProperties::INBOUND_PROPERTY_NAME),
        ]
    }
}

impl From<ConnectorProperties> for NamedProperty {
    fn from(p: ConnectorProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<ConnectorProperties> for String {
    fn from(p: ConnectorProperties) -> Self {
        p.to_string()
    }
}
