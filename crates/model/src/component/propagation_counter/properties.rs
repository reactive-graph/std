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
pub enum PropagationCounterProperties {
    #[strum(serialize = "propagation_count")]
    PROPAGATION_COUNT,
}

impl PropagationCounterProperties {
    pub fn default_value(&self) -> Value {
        match self {
            PropagationCounterProperties::PROPAGATION_COUNT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(PropagationCounterProperties::PROPAGATION_COUNT)]
    }
}

impl From<PropagationCounterProperties> for NamedProperty {
    fn from(p: PropagationCounterProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<PropagationCounterProperties> for String {
    fn from(p: PropagationCounterProperties) -> Self {
        p.to_string()
    }
}
