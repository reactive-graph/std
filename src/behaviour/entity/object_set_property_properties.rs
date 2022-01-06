use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ObjectSetPropertyProperties {
    #[strum(serialize = "object")]
    OBJECT,
    #[strum(serialize = "property_name")]
    PROPERTY_NAME,
    #[strum(serialize = "value")]
    VALUE,
    #[strum(serialize = "result")]
    RESULT,
}

impl ObjectSetPropertyProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ObjectSetPropertyProperties::OBJECT => json!({}),
            ObjectSetPropertyProperties::PROPERTY_NAME => json!(""),
            ObjectSetPropertyProperties::VALUE => json!(0),
            ObjectSetPropertyProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ObjectSetPropertyProperties::OBJECT),
            NamedProperty::from(ObjectSetPropertyProperties::PROPERTY_NAME),
            NamedProperty::from(ObjectSetPropertyProperties::VALUE),
            NamedProperty::from(ObjectSetPropertyProperties::RESULT),
        ]
    }
}

impl From<ObjectSetPropertyProperties> for NamedProperty {
    fn from(p: ObjectSetPropertyProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<ObjectSetPropertyProperties> for String {
    fn from(p: ObjectSetPropertyProperties) -> Self {
        p.to_string()
    }
}
