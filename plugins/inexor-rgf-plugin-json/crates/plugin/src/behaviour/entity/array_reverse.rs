use serde_json::Value;

use crate::model::*;
use crate::model_json::ArrayReverseProperties::ARRAY;
use crate::model_json::ArrayReverseProperties::RESULT;
use crate::reactive::*;

entity_behaviour!(ArrayReverse, ArrayReverseFactory, ArrayReverseFsm, ArrayReverseBehaviourTransitions, ArrayReverseValidator);

behaviour_validator!(ArrayReverseValidator, ReactiveEntityInstance, ARRAY.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ArrayReverseBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(array) = self.reactive_instance.as_array(ARRAY) {
            self.reactive_instance.set(RESULT, reverse_array(&array));
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for ArrayReverseBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(ARRAY.as_ref(), move |array: &Value| {
            if let Some(array) = array.as_array() {
                reactive_instance.set(RESULT, reverse_array(array));
            }
        });
        Ok(())
    }
}

fn reverse_array(array: &Vec<Value>) -> Value {
    Value::Array(array.into_iter().rev().map(|v| v.clone()).collect())
}

impl BehaviourShutdown<ReactiveEntityInstance> for ArrayReverseBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ArrayReverseBehaviourTransitions {}
