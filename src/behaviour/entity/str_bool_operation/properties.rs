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
pub enum StrBoolProperties {
    #[strum(serialize = "lhs")]
    LHS,
    #[strum(serialize = "result")]
    RESULT,
}

impl StrBoolProperties {
    pub fn default_value(&self) -> Value {
        match self {
            StrBoolProperties::LHS => json!(""),
            StrBoolProperties::RESULT => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(StrBoolProperties::LHS), NamedProperty::from(StrBoolProperties::RESULT)]
    }
}

impl From<StrBoolProperties> for NamedProperty {
    fn from(p: StrBoolProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<StrBoolProperties> for String {
    fn from(p: StrBoolProperties) -> Self {
        p.to_string()
    }
}
