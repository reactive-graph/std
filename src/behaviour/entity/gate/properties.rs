use indradb::{Identifier, NamedProperty};
use serde_json::json;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::property::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum NumericGateProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "rhs")]
    RHS,
    #[strum(serialize = "result")]
    RESULT,
}

impl NumericGateProperties {
    pub fn default_value(&self) -> f64 {
        match self {
            NumericGateProperties::LHS => 0.0,
            NumericGateProperties::RHS => 0.0,
            NumericGateProperties::RESULT => 0.0,
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(NumericGateProperties::LHS),
            NamedProperty::from(NumericGateProperties::RHS),
            NamedProperty::from(NumericGateProperties::RESULT),
        ]
    }
}

impl From<NumericGateProperties> for NamedProperty {
    fn from(p: NumericGateProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: json!(p.default_value()),
        }
    }
}
