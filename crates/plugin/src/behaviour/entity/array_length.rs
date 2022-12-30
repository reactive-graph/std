use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_json::ArrayLengthProperties::ARRAY;
use crate::model_json::ArrayLengthProperties::LENGTH;
use crate::reactive::*;

entity_behaviour!(ArrayLength, ArrayLengthFactory, ArrayLengthFsm, ArrayLengthBehaviourTransitions, ArrayLengthValidator);

behaviour_validator!(ArrayLengthValidator, ReactiveEntityInstance, ARRAY.as_ref(), LENGTH.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ArrayLengthBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(array) = self.reactive_instance.as_array(ARRAY) {
            self.reactive_instance.set(LENGTH, json!(array.len()));
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for ArrayLengthBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(ARRAY.as_ref(), move |array: &Value| {
            if let Some(array) = array.as_array() {
                reactive_instance.set(LENGTH, json!(array.len()));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for ArrayLengthBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ArrayLengthBehaviourTransitions {}
