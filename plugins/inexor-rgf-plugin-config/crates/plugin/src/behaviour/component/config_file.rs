use log::error;
use serde_json::Value;
use std::path::Path;

use crate::model::*;
use crate::model_logical::ActionProperties::RESULT;
use crate::model_logical::ActionProperties::TRIGGER;
// TODO: import model_file::FileProperties instead of model_config::ActionProperties
use crate::model_config::FileProperties::FILENAME;
use crate::reactive::*;

entity_behaviour!(ConfigFile, ConfigFileFactory, ConfigFileFsm, ConfigFileBehaviourTransitions, ConfigFileValidator);

behaviour_validator!(ConfigFileValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref(), FILENAME.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ConfigFileBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(data) = self.reactive_instance.as_string(FILENAME).and_then(|filename| read_toml(filename)) {
            self.reactive_instance.set(RESULT, data);
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for ConfigFileBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.is_boolean() || !trigger.as_bool().unwrap_or(false) {
                return;
            }
            if let Some(data) = reactive_instance.as_string(FILENAME).and_then(|filename| read_toml(filename)) {
                reactive_instance.set(RESULT, data);
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for ConfigFileBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ConfigFileBehaviourTransitions {}

fn read_toml(filename: String) -> Option<Value> {
    let filename = shellexpand::tilde(&filename);
    let path = Path::new(filename.as_ref());
    let toml_config = std::fs::read_to_string(path);
    match toml_config {
        Ok(toml_config) => {
            let data = toml::from_str::<Value>(toml_config.as_str());
            match data {
                Ok(data) => Some(data),
                Err(e) => {
                    error!("Failed to parse config file {}: {}", filename, e.to_string());
                    None
                }
            }
        }
        Err(e) => {
            error!("config file {} does not exist: {}", filename, e.to_string());
            None
        }
    }
}
