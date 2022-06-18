use std::convert::AsRef;
use std::sync::Arc;

use crate::behaviour::entity::RandomUuidProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;
use log::trace;
use serde_json::json;
use uuid::Uuid;

pub const RANDOM_UUID: &str = "random_uuid";

pub struct RandomUuid {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl RandomUuid {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<RandomUuid, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.properties.get(RandomUuidProperties::TRIGGER.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(RandomUuidProperties::TRIGGER.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |trigger| {
                    if !trigger.is_boolean() || !trigger.as_bool().unwrap() {
                        return;
                    }
                    entity.set(RandomUuidProperties::RESULT.as_ref(), json!(Uuid::new_v4()));
                },
                handle_id,
            );
        Ok(RandomUuid { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for RandomUuid {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", RANDOM_UUID, self.entity.id);
        let property = self.entity.properties.get(RandomUuidProperties::TRIGGER.as_ref());
        if property.is_some() {
            property.unwrap().stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for RandomUuid {
    fn drop(&mut self) {
        self.disconnect();
    }
}
