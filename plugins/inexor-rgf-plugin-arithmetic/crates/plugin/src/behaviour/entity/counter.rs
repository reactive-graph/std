use serde_json::json;
use serde_json::Value;

use crate::model::*;
// TODO: import model_logical::ActionProperties instead of model_arithmetic::ActionProperties
use crate::model_arithmetic::ActionProperties::RESULT;
use crate::model_arithmetic::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(Counter, CounterFactory, CounterFsm, CounterBehaviourTransitions, CounterValidator);

behaviour_validator!(CounterValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for CounterBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for CounterBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |v: &Value| {
            if !v.is_boolean() {
                // Invalid type
                return;
            }
            if !v.as_bool().unwrap() {
                // Counter only on true (= high)
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
