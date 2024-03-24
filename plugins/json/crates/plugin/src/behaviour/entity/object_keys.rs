use inexor_rgf_behaviour_model_api::behaviour_validator;
use inexor_rgf_behaviour_model_api::prelude::*;
use inexor_rgf_behaviour_model_impl::entity_behaviour;
use inexor_rgf_graph::prelude::*;
use inexor_rgf_reactive_model_impl::ReactiveEntity;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use inexor_rgf_model_json::ObjectKeysProperties::KEYS;
use inexor_rgf_model_json::ObjectKeysProperties::OBJECT;

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
