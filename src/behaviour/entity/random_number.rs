use std::convert::AsRef;
use std::sync::Arc;

use crate::behaviour::entity::RandomNumberProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;
use log::trace;
use rand::Rng;
use serde_json::json;
use uuid::Uuid;

pub const RANDOM_NUMBER: &str = "random_number";

pub struct RandomNumber {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl RandomNumber {
    pub fn new(e: Arc<ReactiveEntityInstance>) -> Result<RandomNumber, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = Uuid::new_v4().as_u128();
        let mut rng = rand::thread_rng();
        e.properties
            .get(RandomNumberProperties::TRIGGER.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |trigger| {
                    if !trigger.is_boolean() || !trigger.as_bool().unwrap() {
                        return;
                    }
                    entity.set(RandomNumberProperties::RESULT.as_ref(), json!(rng.gen::<f64>()));
                },
                handle_id,
            );
        Ok(RandomNumber { entity: e, handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for RandomNumber {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(RandomNumberProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for RandomNumber {
    fn drop(&mut self) {
        self.disconnect();
    }
}
