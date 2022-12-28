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
pub enum ConfigFileProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "filename")]
    FILENAME,
    #[strum(serialize = "result")]
    RESULT,
}

impl ConfigFileProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ConfigFileProperties::TRIGGER => json!(false),
            ConfigFileProperties::FILENAME => json!(""),
            ConfigFileProperties::RESULT => json!({}),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ConfigFileProperties::TRIGGER),
            NamedProperty::from(ConfigFileProperties::FILENAME),
            NamedProperty::from(ConfigFileProperties::RESULT),
        ]
    }
}

impl From<ConfigFileProperties> for NamedProperty {
    fn from(p: ConfigFileProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: json!(p.default_value()),
        }
    }
}

impl From<ConfigFileProperties> for String {
    fn from(p: ConfigFileProperties) -> Self {
        p.to_string()
    }
}
