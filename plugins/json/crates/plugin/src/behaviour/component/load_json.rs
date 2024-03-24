use std::fs::File;
use std::path::Path;

use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use inexor_rgf_runtime_model::ActionProperties::TRIGGER;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_file::FileProperties::FILENAME;
use inexor_rgf_model_result::ResultAnyProperties::RESULT;

entity_behaviour!(LoadJson, LoadJsonFactory, LoadJsonFsm, LoadJsonBehaviourTransitions, LoadJsonValidator);

behaviour_validator!(LoadJsonValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for LoadJsonBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            if let Some(filename) = self.reactive_instance.as_string(FILENAME) {
                if let Some(value) = load_json(filename) {
                    self.reactive_instance.set(RESULT, value);
                }
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for LoadJsonBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            if let Some(filename) = reactive_instance.as_string(FILENAME) {
                if let Some(value) = load_json(filename) {
                    reactive_instance.set(RESULT, value);
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for LoadJsonBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for LoadJsonBehaviourTransitions {}

fn load_json(filename: String) -> Option<Value> {
    match File::open(Path::new(shellexpand::tilde(&filename).as_ref())) {
        Ok(file) => serde_json::from_reader(file).ok(),
        Err(_) => None,
    }
}
