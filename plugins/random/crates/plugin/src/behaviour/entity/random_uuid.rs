use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_std_result_model::ResultStringProperties::RESULT;

entity_behaviour!(RandomUuid, RandomUuidFactory, RandomUuidFsm, RandomUuidBehaviourTransitions, RandomUuidValidator);

behaviour_validator!(RandomUuidValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for RandomUuidBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            self.reactive_instance.set(RESULT, random());
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for RandomUuidBehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for RandomUuidBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for RandomUuidBehaviourTransitions {}

fn random() -> Value {
    json!(Uuid::new_v4())
}
