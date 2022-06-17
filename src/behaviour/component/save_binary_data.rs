use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use log::trace;

use crate::behaviour::component::SaveBinaryDataProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;
use crate::reactive::BehaviourCreationError;

pub const SAVE_BINARY_DATA: &str = "save_binary_data";

pub struct SaveBinaryData {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,
}

impl SaveBinaryData {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<SaveBinaryData, BehaviourCreationError> {
        let entity = e.clone();
        let handle_id = e.id.as_u128();
        let trigger = e.properties.get(SaveBinaryDataProperties::TRIGGER.as_ref()).ok_or(BehaviourCreationError)?;
        trigger.stream.read().unwrap().observe_with_handle(
            move |trigger| {
                if !trigger.is_boolean() || !trigger.as_bool().unwrap_or(false) {
                    return;
                }
                if let Some(data_url) = entity.get(SaveBinaryDataProperties::DATA_URL).and_then(|v| v.as_str().map(String::from)) {
                    if let Some(filename) = entity.get(SaveBinaryDataProperties::FILENAME).and_then(|v| v.as_str().map(String::from)) {
                        let filename = shellexpand::tilde(&filename);
                        let path = Path::new(filename.as_ref());

                        // The next operations may be performance intensive

                        // Decode DataURL with BASE64 encoding to byte array
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
                        let file = OpenOptions::new().write(true).create(true).open(&path);
                        match file {
                            Ok(mut file) => {
                                let _success = file.write_all(bytes.unwrap().as_slice());
                            }
                            Err(_) => {}
                        }
                    }
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
        if let Some(property) = self.entity.properties.get(SaveBinaryDataProperties::TRIGGER.as_ref()) {
            property.stream.read().unwrap().remove(self.handle_id);
        }
    }
}

impl Drop for SaveBinaryData {
    fn drop(&mut self) {
        self.disconnect();
    }
}
