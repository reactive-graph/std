use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum HttpProperties {
    #[strum(serialize = "trigger")]
    TRIGGER,
    #[strum(serialize = "url")]
    URL,
    #[strum(serialize = "method")]
    METHOD,
    #[strum(serialize = "request_headers")]
    REQUEST_HEADERS,
    #[strum(serialize = "payload")]
    PAYLOAD,
    #[strum(serialize = "response_headers")]
    RESPONSE_HEADERS,
    #[strum(serialize = "result")]
    RESULT,
    #[strum(serialize = "status")]
    STATUS,
}

impl HttpProperties {
    pub fn default_value(&self) -> Value {
        match self {
            HttpProperties::TRIGGER => json!(false),
            HttpProperties::URL => json!(""),
            HttpProperties::METHOD => json!("GET"),
            HttpProperties::REQUEST_HEADERS => json!({}),
            HttpProperties::PAYLOAD => json!({}),
            HttpProperties::RESPONSE_HEADERS => json!({}),
            HttpProperties::RESULT => json!({}),
            HttpProperties::STATUS => json!(500),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(HttpProperties::TRIGGER),
            NamedProperty::from(HttpProperties::URL),
            NamedProperty::from(HttpProperties::METHOD),
            NamedProperty::from(HttpProperties::REQUEST_HEADERS),
            NamedProperty::from(HttpProperties::PAYLOAD),
            NamedProperty::from(HttpProperties::RESPONSE_HEADERS),
            NamedProperty::from(HttpProperties::RESULT),
            NamedProperty::from(HttpProperties::STATUS),
        ]
    }
}

impl From<HttpProperties> for NamedProperty {
    fn from(p: HttpProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<HttpProperties> for String {
    fn from(p: HttpProperties) -> Self {
        p.to_string()
    }
}
