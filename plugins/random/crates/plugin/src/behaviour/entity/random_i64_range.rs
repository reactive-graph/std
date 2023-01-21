use rand::Rng;
use serde_json::json;
use serde_json::Value;
use std::ops::Range;

use crate::model::*;
use crate::model_random::RangeI64Properties::HIGH;
use crate::model_random::RangeI64Properties::LOW;
use crate::model_result::ResultNumberI64Properties::RESULT;
use crate::model_trigger::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(
    RandomI64Range,
    RandomI64RangeFactory,
    RandomI64RangeFsm,
    RandomI64RangeBehaviourTransitions,
    RandomI64RangeValidator
);

behaviour_validator!(
    RandomI64RangeValidator,
    ReactiveEntityInstance,
    TRIGGER.as_ref(),
    LOW.as_ref(),
    HIGH.as_ref(),
    RESULT.as_ref()
);

impl BehaviourInit<ReactiveEntityInstance> for RandomI64RangeBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            if let Some(low) = self.reactive_instance.as_i64(LOW) {
                if let Some(high) = self.reactive_instance.as_i64(HIGH) {
                    if low < high {
                        self.reactive_instance.set(RESULT, random(low..high));
                    }
                }
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for RandomI64RangeBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            let Some(low) = reactive_instance.as_i64(LOW) else {
                return;
            };
            let Some(high) = reactive_instance.as_i64(HIGH) else {
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

impl BehaviourShutdown<ReactiveEntityInstance> for RandomI64RangeBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for RandomI64RangeBehaviourTransitions {}

fn random(range: Range<i64>) -> Value {
    let mut rng = rand::thread_rng();
    json!(rng.gen_range(range))
}
