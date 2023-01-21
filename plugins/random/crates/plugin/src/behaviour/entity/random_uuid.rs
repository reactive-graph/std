use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::model::*;
use crate::model_result::ResultStringProperties::RESULT;
use crate::model_trigger::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(RandomUuid, RandomUuidFactory, RandomUuidFsm, RandomUuidBehaviourTransitions, RandomUuidValidator);

behaviour_validator!(RandomUuidValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for RandomUuidBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            self.reactive_instance.set(RESULT, random());
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for RandomUuidBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            reactive_instance.set(RESULT, random());
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for RandomUuidBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for RandomUuidBehaviourTransitions {}

fn random() -> Value {
    json!(Uuid::new_v4())
}
