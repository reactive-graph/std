use crate::reactive::NamedProperties;
use indradb::NamedProperty;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ArrayPopProperties {
    #[strum(serialize = "array")]
    ARRAY,
    #[strum(serialize = "value")]
    VALUE,
    #[strum(serialize = "result")]
    RESULT,
}

impl ArrayPopProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ArrayPopProperties::ARRAY => json!([]),
            ArrayPopProperties::VALUE => json!(0),
            ArrayPopProperties::RESULT => json!([]),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ArrayPopProperties::ARRAY),
            NamedProperty::from(ArrayPopProperties::VALUE),
            NamedProperty::from(ArrayPopProperties::RESULT),
        ]
    }
}

impl From<ArrayPopProperties> for NamedProperty {
    fn from(p: ArrayPopProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<ArrayPopProperties> for String {
    fn from(p: ArrayPopProperties) -> Self {
        p.to_string()
    }
}
