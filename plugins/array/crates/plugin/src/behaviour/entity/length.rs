use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_std_array_model::ArrayLengthProperties::ARRAY;
use reactive_graph_std_array_model::ArrayLengthProperties::LENGTH;

entity_behaviour!(ArrayLength, ArrayLengthFactory, ArrayLengthFsm, ArrayLengthBehaviourTransitions, ArrayLengthValidator);

behaviour_validator!(ArrayLengthValidator, Uuid, ReactiveEntity, ARRAY.as_ref(), LENGTH.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ArrayLengthBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(array) = self.reactive_instance.as_array(ARRAY) {
            self.reactive_instance.set(LENGTH, json!(array.len()));
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ArrayLengthBehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for ArrayLengthBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ArrayLengthBehaviourTransitions {}
