use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_json::ObjectKeysProperties::KEYS;
use crate::model_json::ObjectKeysProperties::OBJECT;
use crate::reactive::*;

entity_behaviour!(ObjectKeys, ObjectKeysFactory, ObjectKeysFsm, ObjectKeysBehaviourTransitions, ObjectKeysValidator);

behaviour_validator!(ObjectKeysValidator, ReactiveEntityInstance, OBJECT.as_ref(), KEYS.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for ObjectKeysBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(object) = self.reactive_instance.get(OBJECT) {
            if let Some(object) = object.as_object() {
                self.reactive_instance.set(KEYS, json!(Vec::from_iter(object.keys())));
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for ObjectKeysBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(OBJECT.as_ref(), move |object: &Value| {
            if let Some(object) = object.as_object() {
                reactive_instance.set(KEYS, json!(Vec::from_iter(object.keys())));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for ObjectKeysBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for ObjectKeysBehaviourTransitions {}
