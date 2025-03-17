use std::ops::Range;

use rand::Rng;
use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_model_random::RangeI64Properties::HIGH;
use reactive_graph_model_random::RangeI64Properties::LOW;
use reactive_graph_model_result::ResultNumberI64Properties::RESULT;

entity_behaviour!(
    RandomI64Range,
    RandomI64RangeFactory,
    RandomI64RangeFsm,
    RandomI64RangeBehaviourTransitions,
    RandomI64RangeValidator
);

behaviour_validator!(RandomI64RangeValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), LOW.as_ref(), HIGH.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for RandomI64RangeBehaviourTransitions {
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

impl BehaviourConnect<Uuid, ReactiveEntity> for RandomI64RangeBehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for RandomI64RangeBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for RandomI64RangeBehaviourTransitions {}

fn random(range: Range<i64>) -> Value {
    let mut rng = rand::rng();
    json!(rng.random_range(range))
}
