use serde_json::Value;

use crate::model::*;
use crate::model_json::ObjectRemovePropertyProperties::OBJECT;
use crate::model_json::ObjectRemovePropertyProperties::PROPERTY_NAME;
use crate::model_json::ObjectRemovePropertyProperties::RESULT;
use crate::reactive::*;

entity_behaviour!(
    ObjectRemoveProperty,
    ObjectRemovePropertyFactory,
    ObjectRemovePropertyFsm,
    ObjectRemovePropertyBehaviourTransitions,
    ObjectRemovePropertyValidator
);

behaviour_validator!(
    ObjectRemovePropertyValidator,
    ReactiveEntityInstance,
    OBJECT.as_ref(),
    RESULT.as_ref(),
    PROPERTY_NAME.as_ref()
);

impl BehaviourInit<ReactiveEntityInstance> for ObjectRemovePropertyBehaviourTransitions {
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

impl BehaviourConnect<ReactiveEntityInstance> for ObjectRemovePropertyBehaviourTransitions {
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

impl BehaviourShutdown<ReactiveEntityInstance> for ObjectRemovePropertyBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ObjectRemovePropertyBehaviourTransitions {}
