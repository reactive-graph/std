use crate::reactive::NamedProperties;
use indradb::NamedProperty;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ObjectGetPropertyProperties {
    #[strum(serialize = "object")]
    OBJECT,
    #[strum(serialize = "property_name")]
    PROPERTY_NAME,
    #[strum(serialize = "result")]
    RESULT,
}

impl ObjectGetPropertyProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ObjectGetPropertyProperties::OBJECT => json!({}),
            ObjectGetPropertyProperties::PROPERTY_NAME => json!(""),
            ObjectGetPropertyProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ObjectGetPropertyProperties::OBJECT),
            NamedProperty::from(ObjectGetPropertyProperties::PROPERTY_NAME),
            NamedProperty::from(ObjectGetPropertyProperties::RESULT),
        ]
    }
}

impl From<ObjectGetPropertyProperties> for NamedProperty {
    fn from(p: ObjectGetPropertyProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<ObjectGetPropertyProperties> for String {
    fn from(p: ObjectGetPropertyProperties) -> Self {
        p.to_string()
    }
}
