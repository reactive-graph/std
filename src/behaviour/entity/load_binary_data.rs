use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use crate::reactive::BehaviourCreationError;
use log::trace;
use serde_json::{json, Value};

use crate::behaviour::entity::LoadBinaryDataProperties;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub const LOAD_BINARY_DATA: &'static str = "load_binary_data";

pub struct LoadBinaryData {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl LoadBinaryData {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<LoadBinaryData, BehaviourCreationError> {
        let handle_id = e.properties.get(LoadBinaryDataProperties::FILENAME.as_ref()).unwrap().id.as_u128();
        let entity = e.clone();
        e.properties
            .get(LoadBinaryDataProperties::FILENAME.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |filename: &Value| {
                    if !filename.is_string() {
                        return;
                    }
                    let filename = filename.as_str().unwrap();
                    let filename = shellexpand::tilde(filename);
                    match File::open(filename.as_ref()) {
                        Ok(mut file) => {
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
                        Err(_) => {}
                    }
                },
                handle_id,
            );
        Ok(LoadBinaryData { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for LoadBinaryData {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", LOAD_BINARY_DATA, self.entity.id);
        let property = self.entity.properties.get(LoadBinaryDataProperties::FILENAME.as_ref());
        if property.is_some() {
            property.unwrap().stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for LoadBinaryData {
    fn drop(&mut self) {
        self.disconnect();
    }
}
