use std::fs::File;

use log::trace;
use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use serde_json::Value;
use uuid::Uuid;

use reactive_graph_model_file::FileProperties::FILENAME;
use reactive_graph_model_json::SaveJsonProperties::PAYLOAD;

entity_behaviour!(SaveJson, SaveJsonFactory, SaveJsonFsm, SaveJsonBehaviourTransitions, SaveJsonValidator);

behaviour_validator!(SaveJsonValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), FILENAME.as_ref(), PAYLOAD.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for SaveJsonBehaviourTransitions {}

impl BehaviourConnect<Uuid, ReactiveEntity> for SaveJsonBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            if let Some(filename) = reactive_instance.get(FILENAME).and_then(|v| v.as_str().map(String::from)) {
                match File::open(&filename) {
                    Ok(file) => {
                        if let Some(value) = reactive_instance.get(PAYLOAD) {
                            if serde_json::to_writer_pretty(file, &value).is_ok() {
                                trace!("Wrote payload to existing file {filename}");
                            }
                        }
                    }
                    Err(_) => {
                        if let Ok(file) = File::create(&filename) {
                            if let Some(value) = reactive_instance.get(PAYLOAD) {
                                if serde_json::to_writer_pretty(file, &value).is_ok() {
                                    trace!("Wrote payload to new file {filename}");
                                }
                            }
                        }
                    }
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for SaveJsonBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for SaveJsonBehaviourTransitions {}
