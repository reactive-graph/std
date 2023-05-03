use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_result::ResultNumberU64Properties::RESULT;
use crate::model_runtime::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(UtcTimestamp, UtcTimestampFactory, UtcTimestampFsm, UtcTimestampBehaviourTransitions, UtcTimestampValidator);

behaviour_validator!(UtcTimestampValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for UtcTimestampBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(trigger) = self.reactive_instance.get(TRIGGER) {
            if trigger.as_bool().unwrap_or(false) {
                self.reactive_instance.set(RESULT, json!(chrono::Utc::now().timestamp()));
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for UtcTimestampBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            reactive_instance.set(RESULT, json!(chrono::Utc::now().timestamp()));
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for UtcTimestampBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for UtcTimestampBehaviourTransitions {}
