use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum LoadBinaryDataProperties {
    #[strum(serialize = "filename")]
    FILENAME,
    #[strum(serialize = "data_url")]
    DATA_URL,
}

impl LoadBinaryDataProperties {
    pub fn default_value(&self) -> Value {
        match self {
            LoadBinaryDataProperties::FILENAME => json!(""),
            LoadBinaryDataProperties::DATA_URL => json!(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(LoadBinaryDataProperties::FILENAME),
            NamedProperty::from(LoadBinaryDataProperties::DATA_URL),
        ]
    }
}

impl From<LoadBinaryDataProperties> for NamedProperty {
    fn from(p: LoadBinaryDataProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: p.default_value(),
        }
    }
}

impl From<LoadBinaryDataProperties> for String {
    fn from(p: LoadBinaryDataProperties) -> Self {
        p.to_string()
    }
}
