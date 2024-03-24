use std::fs::File;

use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use inexor_rgf_runtime_model::ActionProperties::TRIGGER;
use log::trace;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_file::FileProperties::FILENAME;
use inexor_rgf_model_json::SaveJsonProperties::PAYLOAD;

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
                            if let Ok(_) = serde_json::to_writer_pretty(file, &value) {
                                trace!("Wrote payload to existing file {filename}");
                            }
                        }
                    }
                    Err(_) => match File::create(&filename) {
                        Ok(file) => {
                            if let Some(value) = reactive_instance.get(PAYLOAD) {
                                if let Ok(_) = serde_json::to_writer_pretty(file, &value) {
                                    trace!("Wrote payload to new file {filename}");
                                }
                            }
                        }
                        Err(_) => {}
                    },
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for SaveJsonBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for SaveJsonBehaviourTransitions {}
