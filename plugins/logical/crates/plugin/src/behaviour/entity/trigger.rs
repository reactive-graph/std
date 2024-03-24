use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_logical::TriggerProperties::PAYLOAD;
use inexor_rgf_model_result::ResultAnyProperties::RESULT;
use inexor_rgf_runtime_model::ActionProperties::TRIGGER;

entity_behaviour!(Trigger, TriggerFactory, TriggerFsm, TriggerBehaviourTransitions, TriggerValidator);

behaviour_validator!(TriggerValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), RESULT.as_ref(), PAYLOAD.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for TriggerBehaviourTransitions {}

impl BehaviourConnect<Uuid, ReactiveEntity> for TriggerBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            if let Some(payload) = reactive_instance.get(PAYLOAD) {
                reactive_instance.set(RESULT, payload);
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for TriggerBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for TriggerBehaviourTransitions {}
