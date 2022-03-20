use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum RandomBoolProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "result")]
    RESULT,
}

impl RandomBoolProperties {
    pub fn default_value(&self) -> Value {
        match self {
            RandomBoolProperties::TRIGGER => json!(false),
            RandomBoolProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(RandomBoolProperties::TRIGGER),
            NamedProperty::from(RandomBoolProperties::RESULT),
        ]
    }
}

impl From<RandomBoolProperties> for NamedProperty {
    fn from(p: RandomBoolProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<RandomBoolProperties> for String {
    fn from(p: RandomBoolProperties) -> Self {
        p.to_string()
    }
}
