use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum LoadBinaryDataProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "filename")]
    FILENAME,
    #[strum(serialize = "data_url")]
    DATA_URL,
}

impl LoadBinaryDataProperties {
    pub fn default_value(&self) -> Value {
        match self {
            LoadBinaryDataProperties::TRIGGER => json!(false),
            LoadBinaryDataProperties::FILENAME => json!(""),
            LoadBinaryDataProperties::DATA_URL => json!(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(LoadBinaryDataProperties::TRIGGER),
            NamedProperty::from(LoadBinaryDataProperties::FILENAME),
            NamedProperty::from(LoadBinaryDataProperties::DATA_URL),
        ]
    }
}

impl From<LoadBinaryDataProperties> for NamedProperty {
    fn from(p: LoadBinaryDataProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<LoadBinaryDataProperties> for String {
    fn from(p: LoadBinaryDataProperties) -> Self {
        p.to_string()
    }
}
