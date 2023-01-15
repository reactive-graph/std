use serde_json::Value;

use crate::model::*;
use crate::model_logical::ActionProperties::RESULT;
use crate::model_logical::ActionProperties::TRIGGER;
use crate::model_logical::TriggerProperties::PAYLOAD;
use crate::reactive::*;

entity_behaviour!(Trigger, TriggerFactory, TriggerFsm, TriggerBehaviourTransitions, TriggerValidator);

behaviour_validator!(TriggerValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref(), PAYLOAD.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for TriggerBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for TriggerBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.property_observers.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |v: &Value| {
            if !v.is_boolean() {
                // Invalid type
                return;
            }
            if !v.as_bool().unwrap() {
                // Trigger only on true (= high)
                return;
            }
            if let Some(payload) = reactive_instance.get(PAYLOAD) {
                reactive_instance.set(RESULT, payload);
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for TriggerBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for TriggerBehaviourTransitions {}
