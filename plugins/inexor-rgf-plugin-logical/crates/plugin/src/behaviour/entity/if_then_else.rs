use serde_json::Value;

use crate::model::*;
use crate::model_logical::ConditionProperties::CONDITION;
use crate::model_logical::ConditionProperties::RESULT;
use crate::model_logical::IfThenElseProperties::ELSE_PAYLOAD;
use crate::model_logical::IfThenElseProperties::THEN_PAYLOAD;
use crate::reactive::*;

entity_behaviour!(IfThenElse, IfThenElseFactory, IfThenElseFsm, IfThenElseBehaviourTransitions, IfThenElseValidator);

behaviour_validator!(
    IfThenElseValidator,
    ReactiveEntityInstance,
    CONDITION.as_ref(),
    THEN_PAYLOAD.as_ref(),
    ELSE_PAYLOAD.as_ref(),
    RESULT.as_ref()
);

impl BehaviourInit<ReactiveEntityInstance> for IfThenElseBehaviourTransitions {
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

impl BehaviourConnect<ReactiveEntityInstance> for IfThenElseBehaviourTransitions {
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

impl BehaviourShutdown<ReactiveEntityInstance> for IfThenElseBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for IfThenElseBehaviourTransitions {}
