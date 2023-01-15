use rand::Rng;
use serde_json::json;
use serde_json::Value;
use std::ops::Range;

use crate::model::*;
use crate::model_random::RandomF64RangeProperties::HIGH;
use crate::model_random::RandomF64RangeProperties::LOW;
use crate::model_random::RandomF64RangeProperties::RESULT;
use crate::model_random::RandomF64RangeProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(
    RandomF64Range,
    RandomF64RangeFactory,
    RandomF64RangeFsm,
    RandomF64RangeBehaviourTransitions,
    RandomF64RangeValidator
);

behaviour_validator!(
    RandomF64RangeValidator,
    ReactiveEntityInstance,
    TRIGGER.as_ref(),
    LOW.as_ref(),
    HIGH.as_ref(),
    RESULT.as_ref()
);

impl BehaviourInit<ReactiveEntityInstance> for RandomF64RangeBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            if let Some(low) = self.reactive_instance.as_f64(LOW) {
                if let Some(high) = self.reactive_instance.as_f64(HIGH) {
                    if low < high {
                        self.reactive_instance.set(RESULT, random(low..high));
                    }
                }
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for RandomF64RangeBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            let Some(low) = reactive_instance.as_f64(LOW) else {
                return;
            };
            let Some(high) = reactive_instance.as_f64(HIGH) else {
                return;
            };
            if low >= high {
                return;
            }
            reactive_instance.set(RESULT, random(low..high));
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for RandomF64RangeBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for RandomF64RangeBehaviourTransitions {}

fn random(range: Range<f64>) -> Value {
    let mut rng = rand::thread_rng();
    json!(rng.gen_range(range))
}
