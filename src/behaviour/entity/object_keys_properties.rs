use crate::reactive::NamedProperties;
use indradb::NamedProperty;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ObjectKeysProperties {
    #[strum(serialize = "object")]
    OBJECT,
    #[strum(serialize = "keys")]
    KEYS,
}

impl ObjectKeysProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ObjectKeysProperties::OBJECT => json!({}),
            ObjectKeysProperties::KEYS => json!([]),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ObjectKeysProperties::OBJECT),
            NamedProperty::from(ObjectKeysProperties::KEYS),
        ]
    }
}

impl From<ObjectKeysProperties> for NamedProperty {
    fn from(p: ObjectKeysProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<ObjectKeysProperties> for String {
    fn from(p: ObjectKeysProperties) -> Self {
        p.to_string()
    }
}
