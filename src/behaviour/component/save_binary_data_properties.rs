use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum SaveBinaryDataProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "filename")]
    FILENAME,
    #[strum(serialize = "data_url")]
    DATA_URL,
}

impl SaveBinaryDataProperties {
    pub fn default_value(&self) -> Value {
        match self {
            SaveBinaryDataProperties::TRIGGER => json!(false),
            SaveBinaryDataProperties::FILENAME => json!(""),
            SaveBinaryDataProperties::DATA_URL => json!(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(SaveBinaryDataProperties::TRIGGER),
            NamedProperty::from(SaveBinaryDataProperties::FILENAME),
            NamedProperty::from(SaveBinaryDataProperties::DATA_URL),
        ]
    }
}

impl From<SaveBinaryDataProperties> for NamedProperty {
    fn from(p: SaveBinaryDataProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<SaveBinaryDataProperties> for String {
    fn from(p: SaveBinaryDataProperties) -> Self {
        p.to_string()
    }
}
