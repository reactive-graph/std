use crate::reactive::NamedProperties;
use indradb::NamedProperty;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum RandomUuidProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "result")]
    RESULT,
}

impl RandomUuidProperties {
    pub fn default_value(&self) -> Value {
        match self {
            RandomUuidProperties::TRIGGER => json!(false),
            RandomUuidProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(RandomUuidProperties::TRIGGER),
            NamedProperty::from(RandomUuidProperties::RESULT),
        ]
    }
}

impl From<RandomUuidProperties> for NamedProperty {
    fn from(p: RandomUuidProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<RandomUuidProperties> for String {
    fn from(p: RandomUuidProperties) -> Self {
        p.to_string()
    }
}
