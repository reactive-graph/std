use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum PseudoRandomNumberProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "seed")]
    SEED,
    #[strum(serialize = "result")]
    RESULT,
}

impl PseudoRandomNumberProperties {
    pub fn default_value(&self) -> Value {
        match self {
            PseudoRandomNumberProperties::TRIGGER => json!(false),
            PseudoRandomNumberProperties::SEED => json!(0),
            PseudoRandomNumberProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(PseudoRandomNumberProperties::TRIGGER),
            NamedProperty::from(PseudoRandomNumberProperties::SEED),
            NamedProperty::from(PseudoRandomNumberProperties::RESULT),
        ]
    }
}

impl From<PseudoRandomNumberProperties> for NamedProperty {
    fn from(p: PseudoRandomNumberProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<PseudoRandomNumberProperties> for String {
    fn from(p: PseudoRandomNumberProperties) -> Self {
        p.to_string()
    }
}
