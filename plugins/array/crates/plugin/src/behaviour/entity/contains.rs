use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_std_array_model::ArrayContainsProperties::ARRAY;
use reactive_graph_std_array_model::ArrayContainsProperties::SEARCH;
use reactive_graph_std_result_model::ResultArrayProperties::RESULT;

entity_behaviour!(
    ArrayContains,
    ArrayContainsFactory,
    ArrayContainsFsm,
    ArrayContainsBehaviourTransitions,
    ArrayContainsValidator
);

behaviour_validator!(ArrayContainsValidator, Uuid, ReactiveEntity, ARRAY.as_ref(), RESULT.as_ref(), SEARCH.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ArrayContainsBehaviourTransitions {
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

impl BehaviourConnect<Uuid, ReactiveEntity> for ArrayContainsBehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for ArrayContainsBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArrayContainsBehaviourTransitions {}

fn array_contains(array: &Value, search: &Value) -> Option<Value> {
    array.as_array().map(|array| json!(array.contains(search)))
}
