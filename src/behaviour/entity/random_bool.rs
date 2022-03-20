use std::convert::AsRef;
use std::sync::Arc;

use crate::behaviour::entity::RandomBoolProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;
use log::trace;
use rand::Rng;
use serde_json::json;

pub const RANDOM_BOOL: &str = "random_bool";

pub struct RandomBool {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl RandomBool {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<RandomBool, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.properties.get(RandomBoolProperties::TRIGGER.as_ref()).unwrap().id.as_u128();
        let mut rng = rand::thread_rng();
        e.properties
            .get(RandomBoolProperties::TRIGGER.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |trigger| {
                    if !trigger.is_boolean() || !trigger.as_bool().unwrap() {
                        return;
                    }
                    entity.set(RandomBoolProperties::RESULT.as_ref(), json!(rng.gen::<bool>()));
                },
                handle_id,
            );
        Ok(RandomBool { entity: e.clone(), handle_id })
    }
}

impl Disconnectable for RandomBool {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(RandomBoolProperties::TRIGGER.as_ref()) {
            trace!("Disconnecting {} with id {}", RANDOM_BOOL, self.entity.id);
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for RandomBool {
    fn drop(&mut self) {
        self.disconnect();
    }
}
