use serde_json::Value;

use crate::model::*;
use crate::model_json::ArrayPopProperties::ARRAY;
use crate::model_json::ArrayPopProperties::RESULT;
use crate::model_json::ArrayPopProperties::VALUE;
use crate::reactive::*;

entity_behaviour!(ArrayPop, ArrayPopFactory, ArrayPopFsm, ArrayPopBehaviourTransitions, ArrayPopValidator);

behaviour_validator!(ArrayPopValidator, ReactiveEntityInstance, ARRAY.as_ref(), RESULT.as_ref(), VALUE.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ArrayPopBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(array) = self.reactive_instance.get(ARRAY) {
            let (result, value) = pop_array(&array);
            self.reactive_instance.set(RESULT, result);
            if let Some(value) = value {
                self.reactive_instance.set(VALUE, value);
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for ArrayPopBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(ARRAY.as_ref(), move |array: &Value| {
            let (result, value) = pop_array(&array);
            reactive_instance.set(RESULT, result);
            if let Some(value) = value {
                reactive_instance.set(VALUE, value);
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for ArrayPopBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ArrayPopBehaviourTransitions {}

fn pop_array(array: &Value) -> (Value, Option<Value>) {
    match array.as_array() {
        Some(array) => {
            let mut array = array.clone();
            let value = array.pop();
            (Value::Array(array), value)
        }
        None => (array.clone(), None),
    }
}
