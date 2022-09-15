use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::AsRefStr;
use strum_macros::Display;
use strum_macros::IntoStaticStr;

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
    pub fn default_value(&self) -> Value {
        match self {
            StringOperationProperties::LHS => json!(""),
            StringOperationProperties::RESULT => json!(""),
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
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<StringOperationProperties> for String {
    fn from(p: StringOperationProperties) -> Self {
        p.to_string()
    }
}
