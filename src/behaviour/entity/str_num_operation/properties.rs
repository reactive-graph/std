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
pub enum StrNumProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "result")]
    RESULT,
}

impl StrNumProperties {
    pub fn default_value(&self) -> Value {
        match self {
            StrNumProperties::LHS => json!(""),
            StrNumProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(StrNumProperties::LHS), NamedProperty::from(StrNumProperties::RESULT)]
    }
}

impl From<StrNumProperties> for NamedProperty {
    fn from(p: StrNumProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<StrNumProperties> for String {
    fn from(p: StrNumProperties) -> Self {
        p.to_string()
    }
}
