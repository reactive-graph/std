use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::AsRefStr;
use strum_macros::Display;
use strum_macros::IntoStaticStr;

use inexor_rgf_core_reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum ComponentDescribableProperties {
    #[strum(serialize = "name")]
    NAME,
}

impl ComponentDescribableProperties {
    pub fn default_value(&self) -> Value {
        match self {
            ComponentDescribableProperties::NAME => json!(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(ComponentDescribableProperties::NAME)]
    }
}

impl From<ComponentDescribableProperties> for NamedProperty {
    fn from(p: ComponentDescribableProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<ComponentDescribableProperties> for String {
    fn from(p: ComponentDescribableProperties) -> Self {
        p.to_string()
    }
}
