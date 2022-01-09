use crate::reactive::NamedProperties;
use indradb::NamedProperty;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ArrayContainsProperties {
    #[strum(serialize = "array")]
    ARRAY,
    #[strum(serialize = "search")]
    SEARCH,
    #[strum(serialize = "result")]
    RESULT,
}

impl ArrayContainsProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ArrayContainsProperties::ARRAY => json!([]),
            ArrayContainsProperties::SEARCH => json!(0),
            ArrayContainsProperties::RESULT => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ArrayContainsProperties::ARRAY),
            NamedProperty::from(ArrayContainsProperties::SEARCH),
            NamedProperty::from(ArrayContainsProperties::RESULT),
        ]
    }
}

impl From<ArrayContainsProperties> for NamedProperty {
    fn from(p: ArrayContainsProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<ArrayContainsProperties> for String {
    fn from(p: ArrayContainsProperties) -> Self {
        p.to_string()
    }
}
