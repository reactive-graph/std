use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use serde_json::Value;

use crate::model::*;
use crate::model_binary::BinaryDataProperties::DATA_URL;
use crate::model_logical::ActionProperties::RESULT;
use crate::model_logical::ActionProperties::TRIGGER;
// TODO: import model_file::FileProperties instead of model_binary::FileProperties
use crate::model_binary::FileProperties::FILENAME;
use crate::reactive::*;

entity_behaviour!(
    SaveBinaryData,
    SaveBinaryDataFactory,
    SaveBinaryDataFsm,
    SaveBinaryDataBehaviourTransitions,
    SaveBinaryDataValidator
);

behaviour_validator!(SaveBinaryDataValidator, ReactiveEntityInstance, TRIGGER.as_ref(), RESULT.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for SaveBinaryDataBehaviourTransitions {}

impl BehaviourConnect<ReactiveEntityInstance> for SaveBinaryDataBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |v: &Value| {
            if !v.is_boolean() || !v.as_bool().unwrap_or(false) {
                // Invalid type
                return;
            }
            if let Some(filename) = reactive_instance.as_string(FILENAME) {
                let filename = shellexpand::tilde(&filename);
                let path = Path::new(filename.as_ref());

                if let Some(data_url) = reactive_instance.get(DATA_URL).and_then(|v| v.as_str().map(String::from)) {
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
                    if let Ok(mut file) = OpenOptions::new().write(true).create(true).open(&path) {
                        let _success = file.write_all(bytes.unwrap().as_slice());
                    }
                }
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for SaveBinaryDataBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for SaveBinaryDataBehaviourTransitions {}
