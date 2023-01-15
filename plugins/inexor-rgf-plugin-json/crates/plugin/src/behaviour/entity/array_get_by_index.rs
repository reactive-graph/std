use serde_json::Value;

use crate::model::*;
use crate::model_json::ArrayGetByIndexProperties::ARRAY;
use crate::model_json::ArrayGetByIndexProperties::INDEX;
use crate::model_json::ArrayGetByIndexProperties::RESULT;
use crate::reactive::*;

entity_behaviour!(
    ArrayGetByIndex,
    ArrayGetByIndexFactory,
    ArrayGetByIndexFsm,
    ArrayGetByIndexBehaviourTransitions,
    ArrayGetByIndexValidator
);

behaviour_validator!(ArrayGetByIndexValidator, ReactiveEntityInstance, ARRAY.as_ref(), RESULT.as_ref(), INDEX.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ArrayGetByIndexBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(array) = self.reactive_instance.get(ARRAY) {
            if let Some(index) = self.reactive_instance.get(INDEX) {
                if let Some(result) = get_by_index(&array, &index) {
                    self.reactive_instance.set(RESULT, result);
                }
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for ArrayGetByIndexBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(ARRAY.as_ref(), move |array: &Value| {
            if let Some(index) = reactive_instance.get(INDEX) {
                if let Some(result) = get_by_index(array, &index) {
                    reactive_instance.set(RESULT, result);
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(INDEX.as_ref(), move |index: &Value| {
            if let Some(array) = reactive_instance.get(ARRAY) {
                if let Some(result) = get_by_index(&array, index) {
                    reactive_instance.set(RESULT, result);
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for ArrayGetByIndexBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ArrayGetByIndexBehaviourTransitions {}

fn get_by_index(array: &Value, index: &Value) -> Option<Value> {
    match index.as_u64() {
        Some(index) => {
            match array.as_array() {
                Some(array) => match array.get(index as usize) {
                    Some(value) => Some(value.clone()), // PERF: late clone
                    None => None,
                },
                None => None,
            }
        }
        _ => None,
    }
}
