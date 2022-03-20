use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ToggleProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "result")]
    RESULT,
}

impl ToggleProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ToggleProperties::TRIGGER => json!(false),
            ToggleProperties::RESULT => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(ToggleProperties::TRIGGER), NamedProperty::from(ToggleProperties::RESULT)]
    }
}

impl From<ToggleProperties> for NamedProperty {
    fn from(p: ToggleProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<ToggleProperties> for String {
    fn from(p: ToggleProperties) -> Self {
        p.to_string()
    }
}
