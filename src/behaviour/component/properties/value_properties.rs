use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::AsRefStr;
use strum_macros::Display;
use strum_macros::IntoStaticStr;

use crate::reactive::NamedProperties;

pub const COMPONENT_NAME_VALUE: &str = "value";

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ValueProperties {
    #[strum(serialize = "value")]
    VALUE,
}

impl ValueProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ValueProperties::VALUE => json!(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(ValueProperties::VALUE)]
    }
}

impl From<ValueProperties> for NamedProperty {
    fn from(p: ValueProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<ValueProperties> for String {
    fn from(p: ValueProperties) -> Self {
        p.to_string()
    }
}
