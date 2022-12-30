use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::model::*;
use crate::model_random::RandomUuidProperties::RESULT;
use crate::model_random::RandomUuidProperties::TRIGGER;
use crate::reactive::*;

entity_behaviour!(RandomUuid, RandomUuidFactory, RandomUuidFsm, RandomUuidBehaviourTransitions, RandomUuidValidator);

behaviour_validator!(RandomUuidValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for RandomUuidBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            self.reactive_instance.set(RESULT, random());
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for RandomUuidBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                return;
            }
            reactive_instance.set(RESULT, random());
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for RandomUuidBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for RandomUuidBehaviourTransitions {}

fn random() -> Value {
    json!(Uuid::new_v4())
}

// use std::convert::AsRef;
// use std::sync::Arc;
//
// use serde_json::json;
// use uuid::Uuid;
//
// use crate::behaviour::entity::RandomUuidProperties;
// use crate::model::PropertyInstanceSetter;
// use crate::model::ReactiveEntityInstance;
// use crate::reactive::entity::Disconnectable;
// use crate::reactive::BehaviourCreationError;
//
// pub const RANDOM_UUID: &str = "random_uuid";
//
// pub struct RandomUuid {
//     pub entity: Arc<ReactiveEntityInstance>,
//
//     pub handle_id: u128,
// }
//
// impl RandomUuid {
//     pub fn new(e: Arc<ReactiveEntityInstance>) -> Result<RandomUuid, BehaviourCreationError> {
//         let entity = e.clone();
//         let handle_id = Uuid::new_v4().as_u128();
//         e.properties
//             .get(RandomUuidProperties::TRIGGER.as_ref())
//             .unwrap()
//             .stream
//             .read()
//             .unwrap()
//             .observe_with_handle(
//                 move |trigger| {
//                     if !trigger.is_boolean() || !trigger.as_bool().unwrap() {
//                         return;
//                     }
//                     entity.set(RandomUuidProperties::RESULT.as_ref(), json!(Uuid::new_v4()));
//                 },
//                 handle_id,
//             );
//         Ok(RandomUuid { entity: e, handle_id })
//     }
//
//     pub fn type_name(&self) -> String {
//         self.entity.type_name.clone()
//     }
// }
//
// impl Disconnectable for RandomUuid {
//     fn disconnect(&self) {
//         if let Some(property) = self.entity.properties.get(RandomUuidProperties::TRIGGER.as_ref()) {
//             property.stream.read().unwrap().remove(self.handle_id);
//         }
//     }
// }
//
// impl Drop for RandomUuid {
//     fn drop(&mut self) {
//         self.disconnect();
//     }
// }
