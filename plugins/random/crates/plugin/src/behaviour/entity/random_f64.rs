use rand::Rng;
use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_result::ResultNumberF64Properties::RESULT;
use crate::model_trigger::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(RandomF64, RandomF64Factory, RandomF64Fsm, RandomF64BehaviourTransitions, RandomF64Validator);

behaviour_validator!(RandomF64Validator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for RandomF64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            self.reactive_instance.set(RESULT, random());
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for RandomF64BehaviourTransitions {
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

impl BehaviourShutdown<ReactiveEntityInstance> for RandomF64BehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for RandomF64BehaviourTransitions {}

fn random() -> Value {
    let mut rng = rand::thread_rng();
    json!(rng.gen::<f64>())
}
