use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_result::ResultNumberU64Properties::RESULT;
use inexor_rgf_runtime_model::ActionProperties::TRIGGER;

entity_behaviour!(UtcNow, UtcNowFactory, UtcNowFsm, UtcNowBehaviourTransitions, UtcNowValidator);

behaviour_validator!(UtcNowValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for UtcNowBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(trigger) = self.reactive_instance.get(TRIGGER) {
            if trigger.as_bool().unwrap_or(false) {
                self.reactive_instance.set(RESULT, json!(chrono::Utc::now().to_rfc3339()));
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for UtcNowBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            reactive_instance.set(RESULT, json!(chrono::Utc::now().to_rfc3339()));
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for UtcNowBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for UtcNowBehaviourTransitions {}
