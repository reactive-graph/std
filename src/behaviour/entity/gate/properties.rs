use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use strum_macros::AsRefStr;
use strum_macros::Display;
use strum_macros::IntoStaticStr;

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum StringGateProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "rhs")]
    RHS,
    #[strum(serialize = "result")]
    RESULT,
}

impl StringGateProperties {
    pub fn default_value(&self) -> String {
        match self {
            StringGateProperties::LHS => String::new(),
            StringGateProperties::RHS => String::new(),
            StringGateProperties::RESULT => String::new(),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(StringGateProperties::LHS),
            NamedProperty::from(StringGateProperties::RHS),
            NamedProperty::from(StringGateProperties::RESULT),
        ]
    }
}

impl From<StringGateProperties> for NamedProperty {
    fn from(p: StringGateProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: json!(p.default_value()),
        }
    }
}

impl From<StringGateProperties> for String {
    fn from(p: StringGateProperties) -> Self {
        p.to_string()
    }
}
