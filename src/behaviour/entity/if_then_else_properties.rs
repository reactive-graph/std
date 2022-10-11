use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::AsRefStr;
use strum_macros::Display;
use strum_macros::IntoStaticStr;

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum IfThenElseProperties {
    #[strum(serialize = "condition")]
    CONDITION,
    #[strum(serialize = "then_payload")]
    THEN_PAYLOAD,
    #[strum(serialize = "else_payload")]
    ELSE_PAYLOAD,
    #[strum(serialize = "result")]
    RESULT,
}

impl IfThenElseProperties {
    pub fn default_value(&self) -> Value {
        match self {
            IfThenElseProperties::CONDITION => json!(false),
            IfThenElseProperties::THEN_PAYLOAD => json!(0),
            IfThenElseProperties::ELSE_PAYLOAD => json!(0),
            IfThenElseProperties::RESULT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(IfThenElseProperties::CONDITION),
            NamedProperty::from(IfThenElseProperties::THEN_PAYLOAD),
            NamedProperty::from(IfThenElseProperties::ELSE_PAYLOAD),
            NamedProperty::from(IfThenElseProperties::RESULT),
        ]
    }
}

impl From<IfThenElseProperties> for NamedProperty {
    fn from(p: IfThenElseProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<IfThenElseProperties> for String {
    fn from(p: IfThenElseProperties) -> Self {
        p.to_string()
    }
}
