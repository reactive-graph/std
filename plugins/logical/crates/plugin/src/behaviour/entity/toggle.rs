use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_result::ResultBooleanProperties::RESULT;
use inexor_rgf_runtime_model::ActionProperties::TRIGGER;

entity_behaviour!(Toggle, ToggleFactory, ToggleFsm, ToggleBehaviourTransitions, ToggleValidator);

behaviour_validator!(ToggleValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ToggleBehaviourTransitions {}

impl BehaviourConnect<Uuid, ReactiveEntity> for ToggleBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            match reactive_instance.get(RESULT).and_then(|v| v.as_bool()) {
                Some(current_state) => {
                    reactive_instance.set(RESULT, json!(!current_state));
                }
                None => {
                    reactive_instance.set(RESULT, json!(false));
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for ToggleBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ToggleBehaviourTransitions {}
