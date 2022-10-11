use std::convert::AsRef;
use std::sync::Arc;

use log::error;
use serde_json::json;
use serde_json::Value;

use crate::behaviour::entity::toggle_properties::ToggleProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const TOGGLE: &str = "toggle";

pub struct Toggle {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl Toggle {
    pub fn new(e: Arc<ReactiveEntityInstance>) -> Result<Toggle, BehaviourCreationError> {
        let condition = e.properties.get(ToggleProperties::TRIGGER.as_ref());
        if condition.is_none() {
            error!("Missing property {}", ToggleProperties::TRIGGER.as_ref());
            return Err(BehaviourCreationError);
        }
        let result = e.properties.get(ToggleProperties::RESULT.as_ref());
        if result.is_none() {
            error!("Missing property {}", ToggleProperties::RESULT.as_ref());
            return Err(BehaviourCreationError);
        }

        let entity_instance = e.clone();
        let handle_id = e.properties.get(ToggleProperties::TRIGGER.as_ref()).unwrap().id.as_u128();
        e.properties
            .get(ToggleProperties::TRIGGER.as_ref())
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
                        // Toggle only on true (= high)
                        return;
                    }
                    match entity_instance.get(ToggleProperties::RESULT).and_then(|v| v.as_bool()) {
                        Some(current_state) => {
                            entity_instance.set(ToggleProperties::RESULT, json!(!current_state));
                        }
                        None => {
                            entity_instance.set(ToggleProperties::RESULT, json!(false));
                        }
                    }
                },
                handle_id,
            );

        Ok(Toggle { entity: e.clone(), handle_id })
    }
}

impl Disconnectable for Toggle {
    fn disconnect(&self) {
        if let Some(property) = self.entity.properties.get(ToggleProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for Toggle {
    fn drop(&mut self) {
        self.disconnect();
    }
}
