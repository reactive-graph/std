use indradb::NamedProperty;
use serde_json::json;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::property::NamedProperties;

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
            name: p.to_string(),
            value: json!(p.default_value()),
        }
    }
}
