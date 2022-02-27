use std::convert::AsRef;
use std::sync::Arc;

use inexor_rgf_core_model::PropertyInstanceSetter;
use log::error;
use serde_json::Value;

use crate::behaviour::entity::trigger_properties::TriggerProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const TRIGGER: &str = "trigger";

pub struct Trigger {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl Trigger {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<Trigger, BehaviourCreationError> {
        let condition = e.properties.get(TriggerProperties::CONDITION.as_ref());
        if condition.is_none() {
            error!("Missing property condition");
            return Err(BehaviourCreationError);
        }
        let result = e.properties.get(TriggerProperties::RESULT.as_ref());
        if result.is_none() {
            error!("Missing property result");
            return Err(BehaviourCreationError);
        }

        let entity_instance = e.clone();
        let handle_id = e.properties.get(TriggerProperties::CONDITION.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(TriggerProperties::CONDITION.as_ref())
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
                    match entity_instance.get(TriggerProperties::PAYLOAD) {
                        Some(payload) => {
                            entity_instance.set(TriggerProperties::RESULT, payload);
                        }
                        None => {}
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
        if let Some(property) = self.entity.properties.get(TriggerProperties::CONDITION.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for Trigger {
    fn drop(&mut self) {
        self.disconnect();
    }
}
