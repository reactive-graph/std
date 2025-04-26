use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use uuid::Uuid;

use reactive_graph_std_object_model::ObjectSetPropertyProperties::OBJECT;
use reactive_graph_std_object_model::ObjectSetPropertyProperties::PROPERTY_NAME;
use reactive_graph_std_object_model::ObjectSetPropertyProperties::VALUE;
use reactive_graph_std_result_model::ResultAnyProperties::RESULT;

entity_behaviour!(
    ObjectSetProperty,
    ObjectSetPropertyFactory,
    ObjectSetPropertyFsm,
    ObjectSetPropertyBehaviourTransitions,
    ObjectSetPropertyValidator
);

behaviour_validator!(
    ObjectSetPropertyValidator,
    Uuid,
    ReactiveEntity,
    OBJECT.as_ref(),
    RESULT.as_ref(),
    VALUE.as_ref(),
    PROPERTY_NAME.as_ref()
);

impl BehaviourInit<Uuid, ReactiveEntity> for ObjectSetPropertyBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(mut object) = self.reactive_instance.as_object(OBJECT) {
            if let Some(property_name) = self.reactive_instance.as_string(PROPERTY_NAME) {
                if let Some(value) = self.reactive_instance.get(VALUE) {
                    if object.contains_key(&property_name) {
                        object[&property_name] = value;
                    } else {
                        object.insert(property_name, value.clone());
                    }
                    self.reactive_instance.set(RESULT, Value::Object(object));
                }
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ObjectSetPropertyBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(OBJECT.as_ref(), move |object: &Value| {
            if let Some(mut object) = object.as_object().cloned() {
                if let Some(property_name) = reactive_instance.as_string(PROPERTY_NAME) {
                    if let Some(value) = reactive_instance.get(VALUE) {
                        if object.contains_key(&property_name) {
                            object[&property_name] = value;
                        } else {
                            object.insert(property_name, value.clone());
                        }
                        reactive_instance.set(RESULT, Value::Object(object));
                    }
                }
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers
            .observe_with_handle(PROPERTY_NAME.as_ref(), move |property_name: &Value| {
                if let Some(property_name) = property_name.as_str().map(String::from) {
                    if let Some(mut object) = reactive_instance.as_object(OBJECT) {
                        if let Some(value) = reactive_instance.get(VALUE) {
                            if object.contains_key(&property_name) {
                                object[&property_name] = value;
                            } else {
                                object.insert(property_name, value.clone());
                            }
                            reactive_instance.set(RESULT, Value::Object(object));
                        }
                    }
                }
            });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(VALUE.as_ref(), move |value: &Value| {
            if let Some(mut object) = reactive_instance.as_object(OBJECT) {
                if let Some(property_name) = reactive_instance.as_string(PROPERTY_NAME) {
                    if object.contains_key(&property_name) {
                        object[&property_name] = value.clone();
                    } else {
                        object.insert(property_name, value.clone());
                    }
                    reactive_instance.set(RESULT, Value::Object(object));
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for ObjectSetPropertyBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ObjectSetPropertyBehaviourTransitions {}
