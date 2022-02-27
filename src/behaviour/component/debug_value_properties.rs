use indradb::{Identifier, NamedProperty};
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum DebugValueProperties {
    #[strum(serialize = "value")]
    VALUE,
}

impl DebugValueProperties {
    pub fn default_value(&self) -> Value {
        match self {
            DebugValueProperties::VALUE => json!(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(DebugValueProperties::VALUE)]
    }
}

impl From<DebugValueProperties> for NamedProperty {
    fn from(p: DebugValueProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<DebugValueProperties> for String {
    fn from(p: DebugValueProperties) -> Self {
        p.to_string()
    }
}
