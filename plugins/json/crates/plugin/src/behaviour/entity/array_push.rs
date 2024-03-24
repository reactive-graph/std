use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_json::ArrayPushProperties::ARRAY;
use inexor_rgf_model_json::ArrayPushProperties::VALUE;
use inexor_rgf_model_result::ResultArrayProperties::RESULT;

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
