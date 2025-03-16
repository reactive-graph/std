use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

use reactive_graph_model_json::ArrayPopProperties::ARRAY;
use reactive_graph_model_json::ArrayPopProperties::VALUE;
use reactive_graph_model_result::ResultArrayProperties::RESULT;

entity_behaviour!(ArrayPop, ArrayPopFactory, ArrayPopFsm, ArrayPopBehaviourTransitions, ArrayPopValidator);

behaviour_validator!(ArrayPopValidator, Uuid, ReactiveEntity, ARRAY.as_ref(), RESULT.as_ref(), VALUE.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ArrayPopBehaviourTransitions {
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

impl BehaviourConnect<Uuid, ReactiveEntity> for ArrayPopBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(ARRAY.as_ref(), move |array: &Value| {
            let (result, value) = pop_array(array);
            reactive_instance.set(RESULT, result);
            if let Some(value) = value {
                reactive_instance.set(VALUE, value);
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for ArrayPopBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArrayPopBehaviourTransitions {}

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
