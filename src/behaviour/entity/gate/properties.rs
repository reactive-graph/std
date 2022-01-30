use indradb::{Identifier, NamedProperty};
use serde_json::json;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::property::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ArithmeticGateProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "rhs")]
    RHS,
    #[strum(serialize = "result")]
    RESULT,
}

impl ArithmeticGateProperties {
    pub fn default_value(&self) -> f64 {
        match self {
            ArithmeticGateProperties::LHS => 0.0,
            ArithmeticGateProperties::RHS => 0.0,
            ArithmeticGateProperties::RESULT => 0.0,
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ArithmeticGateProperties::LHS),
            NamedProperty::from(ArithmeticGateProperties::RHS),
            NamedProperty::from(ArithmeticGateProperties::RESULT),
        ]
    }
}

impl From<ArithmeticGateProperties> for NamedProperty {
    fn from(p: ArithmeticGateProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: json!(p.default_value()),
        }
    }
}

impl From<ArithmeticGateProperties> for String {
    fn from(p: ArithmeticGateProperties) -> Self {
        p.to_string()
    }
}
