use indradb::NamedProperty;
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::json;
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum ConfigFileProperties {
    #[strum(serialize = "filename")]
    FILENAME,
    #[strum(serialize = "configuration")]
    CONFIGURATION,
}

impl ConfigFileProperties {
    pub fn default_value(&self) -> String {
        match self {
            ConfigFileProperties::FILENAME => String::new(),
            ConfigFileProperties::CONFIGURATION => String::new(),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(ConfigFileProperties::FILENAME),
            NamedProperty::from(ConfigFileProperties::CONFIGURATION),
        ]
    }
}

impl From<ConfigFileProperties> for NamedProperty {
    fn from(p: ConfigFileProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: json!(p.default_value()),
        }
    }
}

impl From<ConfigFileProperties> for String {
    fn from(p: ConfigFileProperties) -> Self {
        p.to_string()
    }
}
