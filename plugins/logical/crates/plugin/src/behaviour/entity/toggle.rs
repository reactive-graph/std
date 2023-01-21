use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_result::ResultBooleanProperties::RESULT;
use crate::model_trigger::ActionProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(Toggle, ToggleFactory, ToggleFsm, ToggleBehaviourTransitions, ToggleValidator);

behaviour_validator!(ToggleValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ToggleBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for ToggleBehaviourTransitions {
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

impl BehaviourShutdown<ReactiveEntityInstance> for ToggleBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ToggleBehaviourTransitions {}
