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
pub enum PropagationCountProperties {
    #[strum(serialize = "propagation_count")]
    PROPAGATION_COUNT,
}

impl PropagationCountProperties {
    pub fn default_value(&self) -> Value {
        match self {
            PropagationCountProperties::PROPAGATION_COUNT => json!(0),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(PropagationCountProperties::PROPAGATION_COUNT)]
    }
}

impl From<PropagationCountProperties> for NamedProperty {
    fn from(p: PropagationCountProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<PropagationCountProperties> for String {
    fn from(p: PropagationCountProperties) -> Self {
        p.to_string()
    }
}
