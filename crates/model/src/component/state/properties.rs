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
pub enum StateProperties {
    #[strum(serialize = "state")]
    STATE,
    #[strum(serialize = "set_state")]
    SET_STATE,
}

impl StateProperties {
    pub fn default_value(&self) -> Value {
        match self {
            StateProperties::STATE => json!(0),
            StateProperties::SET_STATE => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(StateProperties::STATE), NamedProperty::from(StateProperties::SET_STATE)]
    }
}

impl From<StateProperties> for NamedProperty {
    fn from(p: StateProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<StateProperties> for String {
    fn from(p: StateProperties) -> Self {
        p.to_string()
    }
}
