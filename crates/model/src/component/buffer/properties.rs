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
pub enum BufferProperties {
    #[strum(serialize = "buffer_size")]
    BUFFER_SIZE,
    #[strum(serialize = "buffer")]
    BUFFER,
}

impl BufferProperties {
    pub fn default_value(&self) -> Value {
        match self {
            BufferProperties::BUFFER_SIZE => json!(10),
            BufferProperties::BUFFER => json!([]),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(BufferProperties::BUFFER_SIZE),
            NamedProperty::from(BufferProperties::BUFFER),
        ]
    }
}

impl From<BufferProperties> for NamedProperty {
    fn from(p: BufferProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<BufferProperties> for String {
    fn from(p: BufferProperties) -> Self {
        p.to_string()
    }
}
