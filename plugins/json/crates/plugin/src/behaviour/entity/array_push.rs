use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_model_json::ArrayPushProperties::ARRAY;
use reactive_graph_model_json::ArrayPushProperties::VALUE;
use reactive_graph_model_result::ResultArrayProperties::RESULT;

entity_behaviour!(ArrayPush, ArrayPushFactory, ArrayPushFsm, ArrayPushBehaviourTransitions, ArrayPushValidator);

behaviour_validator!(ArrayPushValidator, Uuid, ReactiveEntity, ARRAY.as_ref(), RESULT.as_ref(), VALUE.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ArrayPushBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(array) = self.reactive_instance.get(ARRAY) {
            if let Some(value) = self.reactive_instance.get(VALUE) {
                self.reactive_instance.set(RESULT, push_array(&array, value));
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ArrayPushBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(ARRAY.as_ref(), move |array: &Value| {
            if let Some(value) = reactive_instance.get(VALUE) {
                reactive_instance.set(RESULT, push_array(array, value));
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(VALUE.as_ref(), move |value: &Value| {
            if let Some(array) = reactive_instance.get(ARRAY) {
                reactive_instance.set(RESULT, push_array(&array, value.clone()));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for ArrayPushBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArrayPushBehaviourTransitions {}

fn push_array(array: &Value, value: Value) -> Value {
    match array.as_array() {
        Some(array) => {
            let mut array = array.clone();
            array.push(value);
            Value::Array(array)
        }
        None => json!([value]),
    }
}
