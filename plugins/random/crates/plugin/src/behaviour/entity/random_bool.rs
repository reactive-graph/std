use rand::Rng;
use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_result::ResultBooleanProperties::RESULT;
use crate::model_trigger::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(RandomBool, RandomBoolFactory, RandomBoolFsm, RandomBoolBehaviourTransitions, RandomBoolValidator);

behaviour_validator!(RandomBoolValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for RandomBoolBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            self.reactive_instance.set(RESULT, random());
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for RandomBoolBehaviourTransitions {
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

impl BehaviourShutdown<ReactiveEntityInstance> for RandomBoolBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for RandomBoolBehaviourTransitions {}

fn random() -> Value {
    let mut rng = rand::thread_rng();
    json!(rng.gen::<bool>())
}
