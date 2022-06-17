use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum SaveJsonProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "filename")]
    FILENAME,
    #[strum(serialize = "payload")]
    PAYLOAD,
}

impl SaveJsonProperties {
    pub fn default_value(&self) -> Value {
        match self {
            SaveJsonProperties::TRIGGER => json!(false),
            SaveJsonProperties::FILENAME => json!(""),
            SaveJsonProperties::PAYLOAD => json!({}),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(SaveJsonProperties::TRIGGER),
            NamedProperty::from(SaveJsonProperties::FILENAME),
            NamedProperty::from(SaveJsonProperties::PAYLOAD),
        ]
    }
}

impl From<SaveJsonProperties> for NamedProperty {
    fn from(p: SaveJsonProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<SaveJsonProperties> for String {
    fn from(p: SaveJsonProperties) -> Self {
        p.to_string()
    }
}
