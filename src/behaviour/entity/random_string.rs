use std::convert::AsRef;
use std::sync::Arc;

use log::trace;
use serde_json::json;

use crate::behaviour::entity::RandomStringProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const RANDOM_STRING: &str = "random_string";
pub const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

pub struct RandomString {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl RandomString {
    pub fn new(e: Arc<ReactiveEntityInstance>) -> Result<RandomString, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.properties.get(RandomStringProperties::TRIGGER.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(RandomStringProperties::TRIGGER.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |trigger| {
                    if !trigger.is_boolean() || !trigger.as_bool().unwrap() {
                        return;
                    }
                    let length = entity.get(RandomStringProperties::LENGTH.as_ref()).and_then(|v| v.as_i64());
                    if length.is_none() {
                        return;
                    }
                    let length = length.unwrap() as usize;
                    let rand_str = json!(random_string::generate(length, CHARSET));
                    entity.set(RandomStringProperties::RESULT.as_ref(), rand_str);
                },
                handle_id,
            );
        Ok(RandomString { entity: e.clone(), handle_id })
    }
}

impl Disconnectable for RandomString {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(RandomStringProperties::TRIGGER.as_ref()) {
            trace!("Disconnecting {} with id {}", RANDOM_STRING, self.entity.id);
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for RandomString {
    fn drop(&mut self) {
        self.disconnect();
    }
}
