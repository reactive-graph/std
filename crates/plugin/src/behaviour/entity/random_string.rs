use serde_json::json;
use serde_json::Value;

use crate::model::*;
use crate::model_random::RandomStringProperties::LENGTH;
use crate::model_random::RandomStringProperties::RESULT;
use crate::model_random::RandomStringProperties::TRIGGER;
use crate::reactive::*;

pub const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

entity_behaviour!(RandomString, RandomStringFactory, RandomStringFsm, RandomStringBehaviourTransitions, RandomStringValidator);

behaviour_validator!(RandomStringValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for RandomStringBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            if let Some(length) = self.reactive_instance.as_u64(LENGTH) {
                self.reactive_instance.set(RESULT, random(length));
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for RandomStringBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            if let Some(length) = reactive_instance.as_u64(LENGTH) {
                reactive_instance.set(RESULT, random(length));
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for RandomStringBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for RandomStringBehaviourTransitions {}

fn random(length: u64) -> Value {
    let length = length as usize;
    json!(random_string::generate(length, CHARSET))
}

// use std::convert::AsRef;
// use std::sync::Arc;
//
// use serde_json::json;
// use uuid::Uuid;
//
// use crate::behaviour::entity::RandomStringProperties;
// use crate::model::PropertyInstanceGetter;
// use crate::model::PropertyInstanceSetter;
// use crate::model::ReactiveEntityInstance;
// use crate::reactive::entity::Disconnectable;
// use crate::reactive::BehaviourCreationError;
//
// pub const RANDOM_STRING: &str = "random_string";
// pub const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
//
// pub struct RandomString {
//     pub entity: Arc<ReactiveEntityInstance>,
//
//     pub handle_id: u128,
// }
//
// impl RandomString {
//     pub fn new(e: Arc<ReactiveEntityInstance>) -> Result<RandomString, BehaviourCreationError> {
//         let entity = e.clone();
//         let handle_id = Uuid::new_v4().as_u128();
//         e.properties
//             .get(RandomStringProperties::TRIGGER.as_ref())
//             .unwrap()
//             .stream
//             .read()
//             .unwrap()
//             .observe_with_handle(
//                 move |trigger| {
//                     if !trigger.is_boolean() || !trigger.as_bool().unwrap() {
//                         return;
//                     }
//                     let length = entity.get(RandomStringProperties::LENGTH.as_ref()).and_then(|v| v.as_i64());
//                     if length.is_none() {
//                         return;
//                     }
//                     let length = length.unwrap() as usize;
//                     let rand_str = json!(random_string::generate(length, CHARSET));
//                     entity.set(RandomStringProperties::RESULT.as_ref(), rand_str);
//                 },
//                 handle_id,
//             );
//         Ok(RandomString { entity: e, handle_id })
//     }
// }
//
// impl Disconnectable for RandomString {
//     fn disconnect(&self) {
//         if let Some(property) = self.entity.properties.get(RandomStringProperties::TRIGGER.as_ref()) {
//             property.stream.read().unwrap().remove(self.handle_id);
//         }
//     }
// }
//
// impl Drop for RandomString {
//     fn drop(&mut self) {
//         self.disconnect();
//     }
// }
