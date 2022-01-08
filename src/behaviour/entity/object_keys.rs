use std::convert::AsRef;
use std::sync::Arc;

use crate::model::PropertyInstanceSetter;
use crate::reactive::BehaviourCreationError;
use log::{error, trace};
use serde_json::json;

use crate::behaviour::entity::ObjectKeysProperties;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub const OBJECT_KEYS: &'static str = "object_keys";

pub struct ObjectKeys {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl ObjectKeys {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<ObjectKeys, BehaviourCreationError> {
        let object = e.properties.get(ObjectKeysProperties::OBJECT.as_ref());
        if object.is_none() {
            error!("Missing property {}", ObjectKeysProperties::OBJECT.as_ref());
            return Err(BehaviourCreationError.into());
        }
        let handle_id = object.unwrap().id.as_u128();

        let entity = e.clone();
        e.properties
            .get(ObjectKeysProperties::OBJECT.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |object| {
                    if !object.is_object() {
                        return;
                    }
                    let keys = Vec::from_iter(object.as_object().unwrap().keys());
                    entity.set(ObjectKeysProperties::KEYS.as_ref(), json!(keys));
                },
                handle_id,
            );

        Ok(ObjectKeys { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for ObjectKeys {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", OBJECT_KEYS, self.entity.id);
        match self.entity.properties.get(ObjectKeysProperties::OBJECT.as_ref()) {
            Some(property) => property.stream.read().unwrap().remove(self.handle_id),
            _ => {}
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for ObjectKeys {
    fn drop(&mut self) {
        self.disconnect();
    }
}
