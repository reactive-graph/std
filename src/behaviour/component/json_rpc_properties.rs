use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum JsonRpcProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "url")]
    URL,
    #[strum(serialize = "json_rpc_version")]
    JSON_RPC_VERSION,
    #[strum(serialize = "method")]
    METHOD,
    #[strum(serialize = "params")]
    PARAMS,
    #[strum(serialize = "result")]
    RESULT,
    #[strum(serialize = "error")]
    ERROR,
}

impl JsonRpcProperties {
    pub fn default_value(&self) -> Value {
        match self {
            JsonRpcProperties::TRIGGER => json!(false),
            JsonRpcProperties::URL => json!(""),
            JsonRpcProperties::JSON_RPC_VERSION => json!("2.0"),
            JsonRpcProperties::METHOD => json!(""),
            JsonRpcProperties::PARAMS => json!({}),
            JsonRpcProperties::RESULT => json!({}),
            JsonRpcProperties::ERROR => json!({}),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(JsonRpcProperties::TRIGGER),
            NamedProperty::from(JsonRpcProperties::URL),
            NamedProperty::from(JsonRpcProperties::JSON_RPC_VERSION),
            NamedProperty::from(JsonRpcProperties::METHOD),
            NamedProperty::from(JsonRpcProperties::PARAMS),
            NamedProperty::from(JsonRpcProperties::RESULT),
            NamedProperty::from(JsonRpcProperties::ERROR),
        ]
    }
}

impl From<JsonRpcProperties> for NamedProperty {
    fn from(p: JsonRpcProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<JsonRpcProperties> for String {
    fn from(p: JsonRpcProperties) -> Self {
        p.to_string()
    }
}
