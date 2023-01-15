use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_random::RandomStringProperties::LENGTH;
use crate::model_random::RandomStringProperties::RESULT;
use crate::model_random::RandomStringProperties::TRIGGER;
use crate::reactive::*;

pub const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

entity_behaviour!(RandomString, RandomStringFactory, RandomStringFsm, RandomStringBehaviourTransitions, RandomStringValidator);

behaviour_validator!(RandomStringValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for RandomStringBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            if let Some(length) = self.reactive_instance.as_u64(LENGTH) {
                self.reactive_instance.set(RESULT, random(length));
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for RandomStringBehaviourTransitions {
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

impl BehaviourShutdown<ReactiveEntityInstance> for RandomStringBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for RandomStringBehaviourTransitions {}

fn random(length: u64) -> Value {
    let length = length as usize;
    json!(random_string::generate(length, CHARSET))
}
