use crate::reactive::NamedProperties;
use indradb::NamedProperty;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum RandomNumberProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "result")]
    RESULT,
}

impl RandomNumberProperties {
    pub fn default_value(&self) -> Value {
        match self {
            RandomNumberProperties::TRIGGER => json!(false),
            RandomNumberProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(RandomNumberProperties::TRIGGER),
            NamedProperty::from(RandomNumberProperties::RESULT),
        ]
    }
}

impl From<RandomNumberProperties> for NamedProperty {
    fn from(p: RandomNumberProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<RandomNumberProperties> for String {
    fn from(p: RandomNumberProperties) -> Self {
        p.to_string()
    }
}
