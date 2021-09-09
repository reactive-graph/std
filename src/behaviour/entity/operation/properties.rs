use indradb::NamedProperty;
use serde_json::json;
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

use crate::reactive::property::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum LogicalOperationProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "result")]
    RESULT,
}

impl LogicalOperationProperties {
    pub fn default_value(&self) -> bool {
        match self {
            LogicalOperationProperties::LHS => false,
            LogicalOperationProperties::RESULT => false,
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(LogicalOperationProperties::LHS),
            NamedProperty::from(LogicalOperationProperties::RESULT),
        ]
    }
}

impl From<LogicalOperationProperties> for NamedProperty {
    fn from(p: LogicalOperationProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: json!(p.default_value()),
        }
    }
}

impl From<LogicalOperationProperties> for String {
    fn from(p: LogicalOperationProperties) -> Self {
        p.to_string()
    }
}
