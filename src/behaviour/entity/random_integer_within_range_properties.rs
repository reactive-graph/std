use crate::reactive::NamedProperties;
use indradb::NamedProperty;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum RandomIntegerWithinRangeProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "low")]
    LOW,
    #[strum(serialize = "high")]
    HIGH,
    #[strum(serialize = "result")]
    RESULT,
}

impl RandomIntegerWithinRangeProperties {
    pub fn default_value(&self) -> Value {
        match self {
            RandomIntegerWithinRangeProperties::TRIGGER => json!(false),
            RandomIntegerWithinRangeProperties::LOW => json!(0),
            RandomIntegerWithinRangeProperties::HIGH => json!(0),
            RandomIntegerWithinRangeProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(RandomIntegerWithinRangeProperties::TRIGGER),
            NamedProperty::from(RandomIntegerWithinRangeProperties::LOW),
            NamedProperty::from(RandomIntegerWithinRangeProperties::HIGH),
            NamedProperty::from(RandomIntegerWithinRangeProperties::RESULT),
        ]
    }
}

impl From<RandomIntegerWithinRangeProperties> for NamedProperty {
    fn from(p: RandomIntegerWithinRangeProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<RandomIntegerWithinRangeProperties> for String {
    fn from(p: RandomIntegerWithinRangeProperties) -> Self {
        p.to_string()
    }
}
