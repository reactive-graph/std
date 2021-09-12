use indradb::NamedProperty;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

use crate::reactive::property::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum ComparisonGateProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "rhs")]
    RHS,
    #[strum(serialize = "result")]
    RESULT,
}

impl ComparisonGateProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ComparisonGateProperties::LHS => json!(false),
            ComparisonGateProperties::RHS => json!(false),
            ComparisonGateProperties::RESULT => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ComparisonGateProperties::LHS),
            NamedProperty::from(ComparisonGateProperties::RHS),
            NamedProperty::from(ComparisonGateProperties::RESULT),
        ]
    }
}

impl From<ComparisonGateProperties> for NamedProperty {
    fn from(p: ComparisonGateProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: json!(p.default_value()),
        }
    }
}

impl From<ComparisonGateProperties> for String {
    fn from(p: ComparisonGateProperties) -> Self {
        p.to_string()
    }
}
