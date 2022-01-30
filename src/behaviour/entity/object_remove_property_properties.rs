use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ObjectRemovePropertyProperties {
    #[strum(serialize = "object")]
    OBJECT,
    #[strum(serialize = "property_name")]
    PROPERTY_NAME,
    #[strum(serialize = "result")]
    RESULT,
}

impl ObjectRemovePropertyProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ObjectRemovePropertyProperties::OBJECT => json!({}),
            ObjectRemovePropertyProperties::PROPERTY_NAME => json!(""),
            ObjectRemovePropertyProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ObjectRemovePropertyProperties::OBJECT),
            NamedProperty::from(ObjectRemovePropertyProperties::PROPERTY_NAME),
            NamedProperty::from(ObjectRemovePropertyProperties::RESULT),
        ]
    }
}

impl From<ObjectRemovePropertyProperties> for NamedProperty {
    fn from(p: ObjectRemovePropertyProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<ObjectRemovePropertyProperties> for String {
    fn from(p: ObjectRemovePropertyProperties) -> Self {
        p.to_string()
    }
}
