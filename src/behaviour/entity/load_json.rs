use std::convert::AsRef;
use std::fs::File;
use std::sync::Arc;

use log::trace;

use crate::behaviour::entity::LoadJsonProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const LOAD_JSON: &'static str = "load_json";

pub struct LoadJson {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl LoadJson {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<LoadJson, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.id.as_u128();
        e.properties
            .get(LoadJsonProperties::TRIGGER.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |trigger| {
                    if !trigger.is_boolean() || !trigger.as_bool().unwrap_or(false) {
                        return;
                    }
                    if let Some(filename) = entity.get(LoadJsonProperties::FILENAME).and_then(|v| v.as_str().map(String::from)) {
                        if let Ok(file) = File::open(filename) {
                            if let Ok(value) = serde_json::from_reader(file) {
                                entity.set(LoadJsonProperties::RESULT, value);
                            }
                        }
                    }
                },
                handle_id,
            );
        // Initially load JSON file if trigger is initially true
        if let Some(trigger) = e.properties.get(LoadJsonProperties::TRIGGER.as_ref()) {
            if trigger.get().as_bool().unwrap_or(false) {
                trigger.tick();
            }
        }
        Ok(LoadJson { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for LoadJson {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", LOAD_JSON, self.handle_id);
        if let Some(property) = self.entity.properties.get(LoadJsonProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for LoadJson {
    fn drop(&mut self) {
        self.disconnect();
    }
}
