use inexor_rgf_core_model::PropertyInstanceGetter;
use std::convert::AsRef;
use std::path::Path;
use std::sync::Arc;

use log::error;
use log::trace;
use serde_json::Value;

use crate::behaviour::entity::ConfigFileProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const CONFIG_FILE: &str = "config_file";

pub struct ConfigFile {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl ConfigFile {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<ConfigFile, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.id.as_u128();
        let trigger = e.properties.get(ConfigFileProperties::TRIGGER.as_ref()).ok_or(BehaviourCreationError)?;
        trigger.stream.read().unwrap().observe_with_handle(
            move |trigger| {
                if !trigger.is_boolean() || !trigger.as_bool().unwrap_or(false) {
                    return;
                }
                if let Some(filename) = entity.get(ConfigFileProperties::FILENAME).and_then(|v| v.as_str().map(String::from)) {
                    let filename = shellexpand::tilde(&filename);
                    let path = Path::new(filename.as_ref());
                    let toml_config = std::fs::read_to_string(path);
                    match toml_config {
                        Ok(toml_config) => {
                            let data = toml::from_str::<Value>(toml_config.as_str());
                            match data {
                                Ok(data) => {
                                    entity.set(ConfigFileProperties::RESULT, data.clone());
                                }
                                Err(e) => {
                                    error!("Failed to parse config file {}: {}", filename, e.to_string());
                                }
                            }
                        }
                        Err(e) => {
                            error!("config file {} does not exist: {}", filename, e.to_string());
                        }
                    }
                }
            },
            handle_id,
        );
        // Initially load TOML file if trigger is initially true
        if trigger.get().as_bool().unwrap_or(false) {
            trigger.tick();
        }
        Ok(ConfigFile { entity: e.clone(), handle_id })
    }
}

impl Disconnectable for ConfigFile {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", CONFIG_FILE, self.handle_id);
        if let Some(property) = self.entity.properties.get(ConfigFileProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for ConfigFile {
    fn drop(&mut self) {
        self.disconnect();
    }
}
