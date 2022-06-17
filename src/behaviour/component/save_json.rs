use std::convert::AsRef;
use std::fs::File;
use std::sync::Arc;

use log::trace;

use crate::behaviour::component::SaveJsonProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const SAVE_JSON: &'static str = "save_json";

pub struct SaveJson {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl SaveJson {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<SaveJson, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.id.as_u128();
        e.properties
            .get(SaveJsonProperties::TRIGGER.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |trigger| {
                    if !trigger.is_boolean() || !trigger.as_bool().unwrap_or(false) {
                        return;
                    }
                    if let Some(filename) = entity.get(SaveJsonProperties::FILENAME).and_then(|v| v.as_str().map(String::from)) {
                        match File::open(&filename) {
                            Ok(file) => {
                                if let Some(value) = entity.get(SaveJsonProperties::PAYLOAD) {
                                    if let Ok(_) = serde_json::to_writer_pretty(file, &value) {
                                        trace!("Wrote payload to existing file {filename}");
                                    }
                                }
                            }
                            Err(_) => match File::create(&filename) {
                                Ok(file) => {
                                    if let Some(value) = entity.get(SaveJsonProperties::PAYLOAD) {
                                        if let Ok(_) = serde_json::to_writer_pretty(file, &value) {
                                            trace!("Wrote payload to new file {filename}");
                                        }
                                    }
                                }
                                Err(_) => {}
                            },
                        }
                    }
                },
                handle_id,
            );
        Ok(SaveJson { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for SaveJson {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", SAVE_JSON, self.handle_id);
        if let Some(property) = self.entity.properties.get(SaveJsonProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for SaveJson {
    fn drop(&mut self) {
        self.disconnect();
    }
}
