use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;

use inexor_rgf_core_model::PropertyInstanceGetter;
use log::trace;
use serde_json::json;

use crate::behaviour::component::LoadBinaryDataProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const LOAD_BINARY_DATA: &str = "load_binary_data";

pub struct LoadBinaryData {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl LoadBinaryData {
    pub fn new(e: Arc<ReactiveEntityInstance>) -> Result<LoadBinaryData, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.id.as_u128();
        let trigger = e.properties.get(LoadBinaryDataProperties::TRIGGER.as_ref()).ok_or(BehaviourCreationError)?;
        trigger.stream.read().unwrap().observe_with_handle(
            move |trigger| {
                if !trigger.is_boolean() || !trigger.as_bool().unwrap_or(false) {
                    return;
                }
                if let Some(filename) = entity.get(LoadBinaryDataProperties::FILENAME).and_then(|v| v.as_str().map(String::from)) {
                    let filename = shellexpand::tilde(&filename);
                    let path = Path::new(filename.as_ref());
                    if let Ok(mut file) = File::open(path) {
                        let mut buffer = Vec::new();
                        let _ = file.read_to_end(&mut buffer);
                        match infer::get(&buffer) {
                            Some(kind) => {
                                let mime_type = kind.mime_type();
                                let data_as_base64 = base64::encode(&buffer);
                                let data_url = json!(format!("data:{};base64,{}", mime_type, data_as_base64));
                                entity.set(LoadBinaryDataProperties::DATA_URL.as_ref(), data_url);
                            }
                            None => {}
                        }
                    }
                }
            },
            handle_id,
        );
        // Initially load JSON file if trigger is initially true
        if trigger.get().as_bool().unwrap_or(false) {
            trigger.tick();
        }
        Ok(LoadBinaryData { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for LoadBinaryData {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", LOAD_BINARY_DATA, self.entity.id);
        if let Some(property) = self.entity.properties.get(LoadBinaryDataProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for LoadBinaryData {
    fn drop(&mut self) {
        self.disconnect();
    }
}
