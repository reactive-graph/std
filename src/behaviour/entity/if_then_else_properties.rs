use indradb::{Identifier, NamedProperty};
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

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
