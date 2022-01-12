use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum SaveBinaryDataProperties {
    #[strum(serialize = "filename")]
    FILENAME,
    #[strum(serialize = "data_url")]
    DATA_URL,
}

impl SaveBinaryDataProperties {
    pub fn default_value(&self) -> Value {
        match self {
            SaveBinaryDataProperties::FILENAME => json!(""),
            SaveBinaryDataProperties::DATA_URL => json!(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(SaveBinaryDataProperties::FILENAME),
            NamedProperty::from(SaveBinaryDataProperties::DATA_URL),
        ]
    }
}

impl From<SaveBinaryDataProperties> for NamedProperty {
    fn from(p: SaveBinaryDataProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<SaveBinaryDataProperties> for String {
    fn from(p: SaveBinaryDataProperties) -> Self {
        p.to_string()
    }
}
