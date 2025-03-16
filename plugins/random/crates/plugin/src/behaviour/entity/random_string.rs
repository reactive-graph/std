use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_model_random::RandomStringProperties::LENGTH;
use reactive_graph_model_result::ResultStringProperties::RESULT;

pub const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

entity_behaviour!(RandomString, RandomStringFactory, RandomStringFsm, RandomStringBehaviourTransitions, RandomStringValidator);

behaviour_validator!(RandomStringValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for RandomStringBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            if let Some(length) = self.reactive_instance.as_u64(LENGTH) {
                self.reactive_instance.set(RESULT, random(length));
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for RandomStringBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            if let Some(length) = reactive_instance.as_u64(LENGTH) {
                reactive_instance.set(RESULT, random(length));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for RandomStringBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for RandomStringBehaviourTransitions {}

fn random(length: u64) -> Value {
    let length = length as usize;
    json!(random_string::generate(length, CHARSET))
}
