use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use strum_macros::AsRefStr;
use strum_macros::Display;
use strum_macros::IntoStaticStr;

use crate::reactive::property::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
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
            name: Identifier::new(p.to_string()).unwrap(),
            value: json!(p.default_value()),
        }
    }
}

impl From<LogicalOperationProperties> for String {
    fn from(p: LogicalOperationProperties) -> Self {
        p.to_string()
    }
}
