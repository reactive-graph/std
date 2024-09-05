use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

use reactive_graph_model_logical::TriggerProperties::PAYLOAD;
use reactive_graph_model_result::ResultAnyProperties::RESULT;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;

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
