use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_json::ArrayContainsProperties::ARRAY;
use crate::model_json::ArrayContainsProperties::SEARCH;
use crate::model_result::ResultArrayProperties::RESULT;
use crate::reactive::*;

entity_behaviour!(
    ArrayContains,
    ArrayContainsFactory,
    ArrayContainsFsm,
    ArrayContainsBehaviourTransitions,
    ArrayContainsValidator
);

behaviour_validator!(ArrayContainsValidator, ReactiveEntityInstance, ARRAY.as_ref(), RESULT.as_ref(), SEARCH.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ArrayContainsBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(array) = self.reactive_instance.get(ARRAY) {
            if let Some(search) = self.reactive_instance.get(SEARCH) {
                if let Some(result) = array_contains(&array, &search) {
                    self.reactive_instance.set(RESULT, result);
                }
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for ArrayContainsBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(ARRAY.as_ref(), move |array: &Value| {
            if let Some(search) = reactive_instance.get(SEARCH) {
                if let Some(result) = array_contains(array, &search) {
                    reactive_instance.set(RESULT, result);
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(SEARCH.as_ref(), move |search: &Value| {
            if let Some(array) = reactive_instance.get(ARRAY) {
                if let Some(result) = array_contains(&array, search) {
                    reactive_instance.set(RESULT, result);
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for ArrayContainsBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ArrayContainsBehaviourTransitions {}

fn array_contains(array: &Value, search: &Value) -> Option<Value> {
    match array.as_array() {
        Some(array) => Some(json!(array.contains(search))),
        None => None,
    }
}
