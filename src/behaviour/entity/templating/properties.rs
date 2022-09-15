use crate::reactive::NamedProperties;
use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::AsRefStr;
use strum_macros::Display;
use strum_macros::IntoStaticStr;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum TemplatingProperties {
    #[strum(serialize = "template")]
    TEMPLATE,
    #[strum(serialize = "context")]
    CONTEXT,
    #[strum(serialize = "result")]
    RESULT,
}

impl TemplatingProperties {
    pub fn default_value(&self) -> Value {
        match self {
            TemplatingProperties::TEMPLATE => json!(""),
            TemplatingProperties::CONTEXT => json!({}),
            TemplatingProperties::RESULT => json!(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(TemplatingProperties::TEMPLATE),
            NamedProperty::from(TemplatingProperties::CONTEXT),
            NamedProperty::from(TemplatingProperties::RESULT),
        ]
    }
}

impl From<TemplatingProperties> for NamedProperty {
    fn from(p: TemplatingProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<TemplatingProperties> for String {
    fn from(p: TemplatingProperties) -> Self {
        p.to_string()
    }
}
