use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

use reactive_graph_std_object_model::ObjectGetPropertyProperties::OBJECT;
use reactive_graph_std_object_model::ObjectGetPropertyProperties::PROPERTY_NAME;
use reactive_graph_std_result_model::ResultAnyProperties::RESULT;

entity_behaviour!(
    ObjectGetProperty,
    ObjectGetPropertyFactory,
    ObjectGetPropertyFsm,
    ObjectGetPropertyBehaviourTransitions,
    ObjectGetPropertyValidator
);

behaviour_validator!(ObjectGetPropertyValidator, Uuid, ReactiveEntity, OBJECT.as_ref(), RESULT.as_ref(), PROPERTY_NAME.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ObjectGetPropertyBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(object) = self.reactive_instance.get(OBJECT) {
            if let Some(property_name) = self.reactive_instance.get(PROPERTY_NAME) {
                if let Some(result) = get_property_by_name(&object, &property_name) {
                    self.reactive_instance.set(RESULT, result);
                }
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ObjectGetPropertyBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(OBJECT.as_ref(), move |object: &Value| {
            if let Some(property_name) = reactive_instance.get(PROPERTY_NAME) {
                if let Some(result) = get_property_by_name(object, &property_name) {
                    reactive_instance.set(RESULT, result);
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers
            .observe_with_handle(PROPERTY_NAME.as_ref(), move |property_name: &Value| {
                if let Some(object) = reactive_instance.get(OBJECT) {
                    if let Some(result) = get_property_by_name(&object, property_name) {
                        reactive_instance.set(RESULT, result);
                    }
                }
            });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for ObjectGetPropertyBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ObjectGetPropertyBehaviourTransitions {}

fn get_property_by_name(object: &Value, property_name: &Value) -> Option<Value> {
    match property_name.as_str() {
        Some(property_name) => match object.as_object() {
            Some(object) => object.get(property_name).cloned(),
            None => None,
        },
        _ => None,
    }
}
