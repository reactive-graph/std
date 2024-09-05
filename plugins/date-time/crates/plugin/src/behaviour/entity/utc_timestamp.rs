use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use reactive_graph_model_result::ResultNumberU64Properties::RESULT;

entity_behaviour!(UtcTimestamp, UtcTimestampFactory, UtcTimestampFsm, UtcTimestampBehaviourTransitions, UtcTimestampValidator);

behaviour_validator!(UtcTimestampValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for UtcTimestampBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(trigger) = self.reactive_instance.get(TRIGGER) {
            if trigger.as_bool().unwrap_or(false) {
                self.reactive_instance.set(RESULT, json!(chrono::Utc::now().timestamp()));
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for UtcTimestampBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            reactive_instance.set(RESULT, json!(chrono::Utc::now().timestamp()));
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for UtcTimestampBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for UtcTimestampBehaviourTransitions {}
