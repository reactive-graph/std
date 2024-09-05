use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

use reactive_graph_model_logical::ConditionProperties::CONDITION;
use reactive_graph_model_logical::IfThenElseProperties::ELSE_PAYLOAD;
use reactive_graph_model_logical::IfThenElseProperties::THEN_PAYLOAD;
use reactive_graph_model_result::ResultAnyProperties::RESULT;

entity_behaviour!(IfThenElse, IfThenElseFactory, IfThenElseFsm, IfThenElseBehaviourTransitions, IfThenElseValidator);

behaviour_validator!(
    IfThenElseValidator,
    Uuid,
    ReactiveEntity,
    CONDITION.as_ref(),
    THEN_PAYLOAD.as_ref(),
    ELSE_PAYLOAD.as_ref(),
    RESULT.as_ref()
);

impl BehaviourInit<Uuid, ReactiveEntity> for IfThenElseBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        let condition = self
            .reactive_instance
            .get(CONDITION)
            .and_then(|v| v.as_bool())
            .ok_or(BehaviourInitializationFailed {})?;
        let payload_property = if condition { THEN_PAYLOAD } else { ELSE_PAYLOAD };
        if let Some(payload) = self.reactive_instance.get(payload_property) {
            self.reactive_instance.set(RESULT, payload);
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for IfThenElseBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(CONDITION.as_ref(), move |v: &Value| {
            if !v.is_boolean() {
                // Invalid type, do nothing!
                return;
            }
            let payload_property = if v.as_bool().unwrap() { THEN_PAYLOAD } else { ELSE_PAYLOAD };
            if let Some(payload) = reactive_instance.get(payload_property) {
                reactive_instance.set(RESULT, payload);
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for IfThenElseBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for IfThenElseBehaviourTransitions {}
