use std::path::Path;

use inexor_rgf_behaviour::entity_behaviour;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_model_runtime::ActionProperties::TRIGGER;
use inexor_rgf_reactive::ReactiveEntity;
use log::error;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_file::FileProperties::FILENAME;
use inexor_rgf_model_result::ResultObjectProperties::RESULT;

entity_behaviour!(ConfigFile, ConfigFileFactory, ConfigFileFsm, ConfigFileBehaviourTransitions, ConfigFileValidator);

behaviour_validator!(ConfigFileValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), RESULT.as_ref(), FILENAME.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ConfigFileBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(data) = self.reactive_instance.as_string(FILENAME).and_then(read_toml) {
            self.reactive_instance.set(RESULT, data);
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ConfigFileBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            if let Some(data) = reactive_instance.as_string(FILENAME).and_then(read_toml) {
                reactive_instance.set(RESULT, data);
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for ConfigFileBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ConfigFileBehaviourTransitions {}

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
