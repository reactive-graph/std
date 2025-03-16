use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_model_json::ObjectKeysProperties::KEYS;
use reactive_graph_model_json::ObjectKeysProperties::OBJECT;

entity_behaviour!(ObjectKeys, ObjectKeysFactory, ObjectKeysFsm, ObjectKeysBehaviourTransitions, ObjectKeysValidator);

behaviour_validator!(ObjectKeysValidator, Uuid, ReactiveEntity, OBJECT.as_ref(), KEYS.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for ObjectKeysBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if let Some(object) = self.reactive_instance.get(OBJECT) {
            if let Some(object) = object.as_object() {
                self.reactive_instance.set(KEYS, json!(Vec::from_iter(object.keys())));
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for ObjectKeysBehaviourTransitions {
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

impl BehaviourShutdown<Uuid, ReactiveEntity> for ObjectKeysBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for ObjectKeysBehaviourTransitions {}
