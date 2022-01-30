use indradb::{Identifier, NamedProperty};
use serde_json::json;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::property::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ArithmeticOperationProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "result")]
    RESULT,
}

impl ArithmeticOperationProperties {
    pub fn default_value(&self) -> f64 {
        match self {
            ArithmeticOperationProperties::LHS => 0.0,
            ArithmeticOperationProperties::RESULT => 0.0,
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ArithmeticOperationProperties::LHS),
            NamedProperty::from(ArithmeticOperationProperties::RESULT),
        ]
    }
}

impl From<ArithmeticOperationProperties> for NamedProperty {
    fn from(p: ArithmeticOperationProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: json!(p.default_value()),
        }
    }
}

impl From<ArithmeticOperationProperties> for String {
    fn from(p: ArithmeticOperationProperties) -> Self {
        p.to_string()
    }
}
