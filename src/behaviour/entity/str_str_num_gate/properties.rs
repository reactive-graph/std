use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::AsRefStr;
use strum_macros::Display;
use strum_macros::IntoStaticStr;

use crate::reactive::property::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum StrStrNumProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "rhs")]
    RHS,
    #[strum(serialize = "result")]
    RESULT,
}

impl StrStrNumProperties {
    pub fn default_value(&self) -> Value {
        match self {
            StrStrNumProperties::LHS => json!(""),
            StrStrNumProperties::RHS => json!(""),
            StrStrNumProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(StrStrNumProperties::LHS),
            NamedProperty::from(StrStrNumProperties::RHS),
            NamedProperty::from(StrStrNumProperties::RESULT),
        ]
    }
}

impl From<StrStrNumProperties> for NamedProperty {
    fn from(p: StrStrNumProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}
