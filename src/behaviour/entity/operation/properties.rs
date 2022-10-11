use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use strum_macros::AsRefStr;
use strum_macros::Display;
use strum_macros::IntoStaticStr;

use crate::reactive::property::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum NumericOperationProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "result")]
    RESULT,
}

impl NumericOperationProperties {
    pub fn default_value(&self) -> f64 {
        match self {
            NumericOperationProperties::LHS => 0.0,
            NumericOperationProperties::RESULT => 0.0,
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(NumericOperationProperties::LHS),
            NamedProperty::from(NumericOperationProperties::RESULT),
        ]
    }
}

impl From<NumericOperationProperties> for NamedProperty {
    fn from(p: NumericOperationProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: json!(p.default_value()),
        }
    }
}

impl From<NumericOperationProperties> for String {
    fn from(p: NumericOperationProperties) -> Self {
        p.to_string()
    }
}
