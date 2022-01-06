use indradb::NamedProperty;
use serde_json::json;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::property::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum StringOperationProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "result")]
    RESULT,
}

impl StringOperationProperties {
    pub fn default_value(&self) -> String {
        match self {
            StringOperationProperties::LHS => String::new(),
            StringOperationProperties::RESULT => String::new(),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(StringOperationProperties::LHS),
            NamedProperty::from(StringOperationProperties::RESULT),
        ]
    }
}

impl From<StringOperationProperties> for NamedProperty {
    fn from(p: StringOperationProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: json!(p.default_value()),
        }
    }
}

impl From<StringOperationProperties> for String {
    fn from(p: StringOperationProperties) -> Self {
        p.to_string()
    }
}
