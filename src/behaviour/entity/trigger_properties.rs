use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum TriggerProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "payload")]
    PAYLOAD,
    #[strum(serialize = "result")]
    RESULT,
}

impl TriggerProperties {
    pub fn default_value(&self) -> Value {
        match self {
            TriggerProperties::TRIGGER => json!(false),
            TriggerProperties::PAYLOAD => json!(false),
            TriggerProperties::RESULT => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(TriggerProperties::TRIGGER),
            NamedProperty::from(TriggerProperties::PAYLOAD),
            NamedProperty::from(TriggerProperties::RESULT),
        ]
    }
}

impl From<TriggerProperties> for NamedProperty {
    fn from(p: TriggerProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<TriggerProperties> for String {
    fn from(p: TriggerProperties) -> Self {
        p.to_string()
    }
}
