use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ArrayGetByIndexProperties {
    #[strum(serialize = "array")]
    ARRAY,
    #[strum(serialize = "index")]
    INDEX,
    #[strum(serialize = "result")]
    RESULT,
}

impl ArrayGetByIndexProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ArrayGetByIndexProperties::ARRAY => json!([]),
            ArrayGetByIndexProperties::INDEX => json!(0),
            ArrayGetByIndexProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ArrayGetByIndexProperties::ARRAY),
            NamedProperty::from(ArrayGetByIndexProperties::INDEX),
            NamedProperty::from(ArrayGetByIndexProperties::RESULT),
        ]
    }
}

impl From<ArrayGetByIndexProperties> for NamedProperty {
    fn from(p: ArrayGetByIndexProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<ArrayGetByIndexProperties> for String {
    fn from(p: ArrayGetByIndexProperties) -> Self {
        p.to_string()
    }
}
