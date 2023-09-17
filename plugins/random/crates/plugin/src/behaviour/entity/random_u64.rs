use inexor_rgf_behaviour::entity_behaviour;
use inexor_rgf_behaviour_api::behaviour_validator;
use inexor_rgf_behaviour_api::prelude::*;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_model_runtime::ActionProperties::TRIGGER;
use inexor_rgf_reactive::ReactiveEntity;
use rand::Rng;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_result::ResultNumberU64Properties::RESULT;

entity_behaviour!(RandomU64, RandomU64Factory, RandomU64Fsm, RandomU64BehaviourTransitions, RandomU64Validator);

behaviour_validator!(RandomU64Validator, Uuid, ReactiveEntity, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for RandomU64BehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            self.reactive_instance.set(RESULT, random());
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for RandomU64BehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for RandomU64BehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for RandomU64BehaviourTransitions {}

fn random() -> Value {
    let mut rng = rand::thread_rng();
    json!(rng.gen::<u64>())
}
