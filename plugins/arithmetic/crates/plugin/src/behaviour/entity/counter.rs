use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_result::ResultNumberU64Properties::RESULT;
use crate::model_trigger::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(Counter, CounterFactory, CounterFsm, CounterBehaviourTransitions, CounterValidator);

behaviour_validator!(CounterValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for CounterBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for CounterBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        // let counter = crate::model_arithmetic::Counter::from(self.reactive_instance.clone());
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            match reactive_instance.get(RESULT).and_then(|v| v.as_i64()) {
                Some(current_value) => {
                    reactive_instance.set(RESULT, json!(current_value + 1));
                }
                None => {
                    reactive_instance.set(RESULT, json!(0));
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for CounterBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for CounterBehaviourTransitions {}
