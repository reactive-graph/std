use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum LoadJsonProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "filename")]
    FILENAME,
    #[strum(serialize = "result")]
    RESULT,
}

impl LoadJsonProperties {
    pub fn default_value(&self) -> Value {
        match self {
            LoadJsonProperties::TRIGGER => json!(false),
            LoadJsonProperties::FILENAME => json!(""),
            LoadJsonProperties::RESULT => json!({}),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(LoadJsonProperties::TRIGGER),
            NamedProperty::from(LoadJsonProperties::FILENAME),
            NamedProperty::from(LoadJsonProperties::RESULT),
        ]
    }
}

impl From<LoadJsonProperties> for NamedProperty {
    fn from(p: LoadJsonProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<LoadJsonProperties> for String {
    fn from(p: LoadJsonProperties) -> Self {
        p.to_string()
    }
}
