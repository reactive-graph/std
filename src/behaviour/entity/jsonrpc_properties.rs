use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum JsonRpcProperties {
    #[strum(serialize = "url")]
    URL,
    #[strum(serialize = "jsonrpc_version")]
    JSONRPC_VERSION,
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
            JsonRpcProperties::URL => json!(""),
            JsonRpcProperties::JSONRPC_VERSION => json!("2.0"),
            JsonRpcProperties::METHOD => json!(""),
            JsonRpcProperties::PARAMS => json!({}),
            JsonRpcProperties::RESULT => json!({}),
            JsonRpcProperties::ERROR => json!({}),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(JsonRpcProperties::URL),
            NamedProperty::from(JsonRpcProperties::JSONRPC_VERSION),
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
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<JsonRpcProperties> for String {
    fn from(p: JsonRpcProperties) -> Self {
        p.to_string()
    }
}
