use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum CounterProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "result")]
    RESULT,
}

impl CounterProperties {
    pub fn default_value(&self) -> Value {
        match self {
            CounterProperties::TRIGGER => json!(false),
            CounterProperties::RESULT => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(CounterProperties::TRIGGER), NamedProperty::from(CounterProperties::RESULT)]
    }
}

impl From<CounterProperties> for NamedProperty {
    fn from(p: CounterProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<CounterProperties> for String {
    fn from(p: CounterProperties) -> Self {
        p.to_string()
    }
}
