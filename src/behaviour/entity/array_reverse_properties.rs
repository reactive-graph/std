use crate::reactive::NamedProperties;
use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ArrayReverseProperties {
    #[strum(serialize = "array")]
    ARRAY,
    #[strum(serialize = "result")]
    RESULT,
}

impl ArrayReverseProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ArrayReverseProperties::ARRAY => json!([]),
            ArrayReverseProperties::RESULT => json!([]),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ArrayReverseProperties::ARRAY),
            NamedProperty::from(ArrayReverseProperties::RESULT),
        ]
    }
}

impl From<ArrayReverseProperties> for NamedProperty {
    fn from(p: ArrayReverseProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<ArrayReverseProperties> for String {
    fn from(p: ArrayReverseProperties) -> Self {
        p.to_string()
    }
}
