use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ArrayLengthProperties {
    #[strum(serialize = "array")]
    ARRAY,
    #[strum(serialize = "length")]
    LENGTH,
}

impl ArrayLengthProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ArrayLengthProperties::ARRAY => json!([]),
            ArrayLengthProperties::LENGTH => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ArrayLengthProperties::ARRAY),
            NamedProperty::from(ArrayLengthProperties::LENGTH),
        ]
    }
}

impl From<ArrayLengthProperties> for NamedProperty {
    fn from(p: ArrayLengthProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<ArrayLengthProperties> for String {
    fn from(p: ArrayLengthProperties) -> Self {
        p.to_string()
    }
}
