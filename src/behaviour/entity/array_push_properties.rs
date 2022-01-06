use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ArrayPushProperties {
    #[strum(serialize = "array")]
    ARRAY,
    #[strum(serialize = "value")]
    VALUE,
    #[strum(serialize = "result")]
    RESULT,
}

impl ArrayPushProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ArrayPushProperties::ARRAY => json!([]),
            ArrayPushProperties::VALUE => json!(0),
            ArrayPushProperties::RESULT => json!([]),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ArrayPushProperties::ARRAY),
            NamedProperty::from(ArrayPushProperties::VALUE),
            NamedProperty::from(ArrayPushProperties::RESULT),
        ]
    }
}

impl From<ArrayPushProperties> for NamedProperty {
    fn from(p: ArrayPushProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<ArrayPushProperties> for String {
    fn from(p: ArrayPushProperties) -> Self {
        p.to_string()
    }
}
