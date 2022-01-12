use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;

use crate::reactive::BehaviourCreationError;
use log::trace;
use serde_json::Value;

use crate::behaviour::entity::{LoadBinaryDataProperties, SaveBinaryDataProperties};
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub const SAVE_BINARY_DATA: &'static str = "save_binary_data";

pub struct SaveBinaryData {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl SaveBinaryData {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<SaveBinaryData, BehaviourCreationError> {
        let handle_id = e.properties.get(LoadBinaryDataProperties::DATA_URL.as_ref()).unwrap().id.as_u128();
        let entity = e.clone();
        e.properties
            .get(LoadBinaryDataProperties::DATA_URL.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |data_url: &Value| {
                    if !data_url.is_string() {
                        return;
                    }

                    // Resolve the filename
                    let filename = match entity.get(SaveBinaryDataProperties::FILENAME.as_ref()) {
                        Some(filename) => {
                            if filename.is_string() {
                                Some(String::from(shellexpand::tilde(filename.as_str().unwrap())))
                            } else {
                                None
                            }
                        }
                        None => None,
                    };
                    if filename.is_none() {
                        return;
                    }

                    // The next operations may be performance intensive

                    // Decode DataURL with BASE64 encoding to byte array
                    let data_url = data_url.as_str().unwrap();
                    let mut parts = data_url.splitn(2, ',');
                    let _part_data_url_prefix = parts.next();
                    let bytes = match parts.next() {
                        Some(part_base64_encoded_data) => match base64::decode(part_base64_encoded_data) {
                            Ok(bytes) => Some(bytes),
                            Err(_) => None,
                        },
                        None => None,
                    };
                    if bytes.is_none() {
                        return;
                    }

                    // Write byte array to file
                    let file = OpenOptions::new().write(true).create(true).open(filename.unwrap());
                    match file {
                        Ok(mut file) => {
                            let _success = file.write_all(bytes.unwrap().as_slice());
                        }
                        Err(_) => {}
                    }
                },
                handle_id,
            );
        Ok(SaveBinaryData { entity: e.clone(), handle_id })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for SaveBinaryData {
    fn disconnect(&self) {
        trace!("Disconnecting {} with id {}", SAVE_BINARY_DATA, self.entity.id);
        let property = self.entity.properties.get(LoadBinaryDataProperties::DATA_URL.as_ref());
        if property.is_some() {
            property.unwrap().stream.read().unwrap().remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for SaveBinaryData {
    fn drop(&mut self) {
        self.disconnect();
    }
}
