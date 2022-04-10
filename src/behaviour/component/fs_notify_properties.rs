use indradb::Identifier;
use indradb::NamedProperty;
use serde_json::json;
use serde_json::Value;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::NamedProperties;

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum FsNotifyProperties {
    #[strum(serialize = "filename")]
    FILENAME,
    #[strum(serialize = "trigger")]
    TRIGGER,
}

impl FsNotifyProperties {
    pub fn default_value(&self) -> Value {
        match self {
            FsNotifyProperties::FILENAME => json!(""),
            FsNotifyProperties::TRIGGER => json!(false),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(FsNotifyProperties::FILENAME),
            NamedProperty::from(FsNotifyProperties::TRIGGER),
        ]
    }
}

impl From<FsNotifyProperties> for NamedProperty {
    fn from(p: FsNotifyProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: p.default_value(),
        }
    }
}

impl From<FsNotifyProperties> for String {
    fn from(p: FsNotifyProperties) -> Self {
        p.to_string()
    }
}
