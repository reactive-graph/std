use std::convert::AsRef;
use std::path::Path;
use std::sync::Arc;

use crate::reactive::BehaviourCreationError;
use log::{debug, error};
use serde_json::Value;

use crate::behaviour::entity::ConfigFileProperties;
use crate::model::{PropertyInstanceSetter, ReactiveEntityInstance};
use crate::reactive::entity::Disconnectable;

pub struct ConfigFile {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl ConfigFile {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<ConfigFile, BehaviourCreationError> {
        let filename = e.properties.get(ConfigFileProperties::FILENAME.as_ref());
        if filename.is_none() {
            return Err(BehaviourCreationError.into());
        }
        let filename = filename.unwrap();
        let handle_id = filename.id.as_u128();

        let config_file = e.clone();

        filename.stream.read().unwrap().observe_with_handle(
            move |filename| {
                let filename = filename.as_str();
                if filename.is_none() {
                    debug!("filename is not a string");
                    return;
                }
                let filename = filename.unwrap();
                if filename.is_empty() {
                    debug!("filename is empty");
                    return;
                }
                let filename = shellexpand::tilde(filename);
                let path = Path::new(filename.as_ref());

                debug!(
                    "Reading config file {:?} into entity instance {}",
                    path, config_file.id
                );

                let toml_config = std::fs::read_to_string(path);
                match toml_config {
                    Ok(toml_config) => {
                        let data = toml::from_str::<Value>(toml_config.as_str());
                        match data {
                            Ok(data) => {
                                config_file.set(ConfigFileProperties::CONFIGURATION, data.clone());
                            }
                            Err(e) => {
                                error!(
                                    "Failed to parse config file {}: {}",
                                    filename,
                                    e.to_string()
                                );
                            }
                        }
                    }
                    Err(e) => {
                        error!("config file {} does not exist: {}", filename, e.to_string());
                    }
                }
            },
            handle_id,
        );

        Ok(ConfigFile {
            entity: e.clone(),
            handle_id,
        })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ConfigFile {
    fn disconnect(&self) {
        debug!("Disconnecting config file {}", self.handle_id);
        let property = self
            .entity
            .properties
            .get(ConfigFileProperties::FILENAME.as_ref());
        if property.is_some() {
            property
                .unwrap()
                .stream
                .read()
                .unwrap()
                .remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ConfigFile {
    fn drop(&mut self) {
        self.disconnect();
    }
}
