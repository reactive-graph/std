use indradb::{Identifier, NamedProperty};
use serde_json::json;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::property::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum StringComparisonProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "rhs")]
    RHS,
    #[strum(serialize = "result")]
    RESULT,
}

impl StringComparisonProperties {
    pub fn default_value(&self) -> String {
        match self {
            StringComparisonProperties::LHS => String::new(),
            StringComparisonProperties::RHS => String::new(),
            StringComparisonProperties::RESULT => String::new(),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(StringComparisonProperties::LHS),
            NamedProperty::from(StringComparisonProperties::RHS),
            NamedProperty::from(StringComparisonProperties::RESULT),
        ]
    }
}

impl From<StringComparisonProperties> for NamedProperty {
    fn from(p: StringComparisonProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: json!(p.default_value()),
        }
    }
}

impl From<StringComparisonProperties> for String {
    fn from(p: StringComparisonProperties) -> Self {
        p.to_string()
    }
}
