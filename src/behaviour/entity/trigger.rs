use std::convert::AsRef;
use std::sync::Arc;

use log::error;
use serde_json::Value;

use crate::behaviour::entity::trigger_properties::TriggerProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const TRIGGER: &str = "trigger";

pub struct Trigger {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl Trigger {
    pub fn new(e: Arc<ReactiveEntityInstance>) -> Result<Trigger, BehaviourCreationError> {
        let trigger = e.properties.get(TriggerProperties::TRIGGER.as_ref());
        if trigger.is_none() {
            error!("Missing property {}", TriggerProperties::TRIGGER.as_ref());
            return Err(BehaviourCreationError);
        }
        let result = e.properties.get(TriggerProperties::RESULT.as_ref());
        if result.is_none() {
            error!("Missing property {}", TriggerProperties::RESULT.as_ref());
            return Err(BehaviourCreationError);
        }

        let entity_instance = e.clone();
        let handle_id = e.properties.get(TriggerProperties::TRIGGER.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(TriggerProperties::TRIGGER.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |v: &Value| {
                    if !v.is_boolean() {
                        // Invalid type
                        return;
                    }
                    if !v.as_bool().unwrap() {
                        // Trigger only on true (= high)
                        return;
                    }
                    if let Some(payload) = entity_instance.get(TriggerProperties::PAYLOAD) {
                        entity_instance.set(TriggerProperties::RESULT, payload);
                    }
                },
                handle_id,
            );

        Ok(Trigger { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for Trigger {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(TriggerProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for Trigger {
    fn drop(&mut self) {
        self.disconnect();
    }
}
