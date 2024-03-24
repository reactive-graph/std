use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use inexor_rgf_runtime_model::ActionProperties::TRIGGER;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_random::RandomStringProperties::LENGTH;
use inexor_rgf_model_result::ResultStringProperties::RESULT;

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
