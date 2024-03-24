use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_json::ObjectRemovePropertyProperties::OBJECT;
use inexor_rgf_model_json::ObjectRemovePropertyProperties::PROPERTY_NAME;
use inexor_rgf_model_result::ResultAnyProperties::RESULT;

entity_behaviour!(
    ObjectRemoveProperty,
    ObjectRemovePropertyFactory,
    ObjectRemovePropertyFsm,
    ObjectRemovePropertyBehaviourTransitions,
    ObjectRemovePropertyValidator
);

behaviour_validator!(ObjectRemovePropertyValidator, Uuid, ReactiveEntity, OBJECT.as_ref(), RESULT.as_ref(), PROPERTY_NAME.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ObjectRemovePropertyBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(mut object) = self.reactive_instance.as_object(OBJECT) {
            if let Some(property_name) = self.reactive_instance.as_string(PROPERTY_NAME) {
                object.remove(&property_name);
                self.reactive_instance.set(RESULT, Value::Object(object));
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ObjectRemovePropertyBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(OBJECT.as_ref(), move |object: &Value| {
            if let Some(mut object) = object.as_object().cloned() {
                if let Some(property_name) = reactive_instance.as_string(PROPERTY_NAME) {
                    object.remove(&property_name);
                    reactive_instance.set(RESULT, Value::Object(object));
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers
            .observe_with_handle(PROPERTY_NAME.as_ref(), move |property_name: &Value| {
                if let Some(property_name) = property_name.as_str().map(String::from) {
                    if let Some(mut object) = reactive_instance.as_object(OBJECT) {
                        object.remove(&property_name);
                        reactive_instance.set(RESULT, Value::Object(object));
                    }
                }
            });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for ObjectRemovePropertyBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ObjectRemovePropertyBehaviourTransitions {}
