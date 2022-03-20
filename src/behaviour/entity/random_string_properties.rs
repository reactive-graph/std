use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum RandomStringProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "length")]
    LENGTH,
    #[strum(serialize = "result")]
    RESULT,
}

impl RandomStringProperties {
    pub fn default_value(&self) -> Value {
        match self {
            RandomStringProperties::TRIGGER => json!(false),
            RandomStringProperties::LENGTH => json!(8),
            RandomStringProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(RandomStringProperties::TRIGGER),
            NamedProperty::from(RandomStringProperties::LENGTH),
            NamedProperty::from(RandomStringProperties::RESULT),
        ]
    }
}

impl From<RandomStringProperties> for NamedProperty {
    fn from(p: RandomStringProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<RandomStringProperties> for String {
    fn from(p: RandomStringProperties) -> Self {
        p.to_string()
    }
}
