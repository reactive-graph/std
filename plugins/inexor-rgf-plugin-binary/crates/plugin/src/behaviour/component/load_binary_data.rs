use std::fs::File;
use std::io::Read;
use std::path::Path;

use mime_guess::from_path;
use serde_json::json;
use serde_json::Value;

use crate::model::*;
// TODO: import model_logical::ActionProperties instead of model_binary::ActionProperties
use crate::model_binary::ActionProperties::TRIGGER;
use crate::model_binary::BinaryDataProperties::DATA_URL;
// TODO: import model_file::FileProperties instead of model_binary::FileProperties
use crate::model_binary::FileProperties::FILENAME;
use crate::reactive::*;

entity_behaviour!(
    LoadBinaryData,
    LoadBinaryDataFactory,
    LoadBinaryDataFsm,
    LoadBinaryDataBehaviourTransitions,
    LoadBinaryDataValidator
);

behaviour_validator!(LoadBinaryDataValidator, ReactiveEntityInstance, TRIGGER.as_ref(), FILENAME.as_ref(), DATA_URL.as_ref());

impl BehaviourInit<ReactiveEntityInstance> for LoadBinaryDataBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            if let Some(value) = self.reactive_instance.as_string(FILENAME).and_then(|filename| load_binary_data(filename)) {
                self.reactive_instance.set(DATA_URL, value);
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<ReactiveEntityInstance> for LoadBinaryDataBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.is_boolean() || !trigger.as_bool().unwrap_or(false) {
                // Invalid type
                return;
            }
            if let Some(value) = reactive_instance.as_string(FILENAME).and_then(|filename| load_binary_data(filename)) {
                reactive_instance.set(DATA_URL, value);
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(FILENAME.as_ref(), move |filename: &Value| {
            if let Some(value) = filename.as_str().map(String::from).and_then(|filename| load_binary_data(filename)) {
                reactive_instance.set(DATA_URL, value);
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<ReactiveEntityInstance> for LoadBinaryDataBehaviourTransitions {}
impl BehaviourTransitions<ReactiveEntityInstance> for LoadBinaryDataBehaviourTransitions {}

fn load_binary_data(filename: String) -> Option<Value> {
    let filename = shellexpand::tilde(&filename);
    let path = Path::new(filename.as_ref());
    File::open(path).ok().and_then(|mut file| {
        let mut buffer = Vec::new();
        let _ = file.read_to_end(&mut buffer);
        infer::get(&buffer)
            .map(|kind| kind.mime_type().to_string())
            .or_else(|| from_path(path).first().map(|x| x.to_string()))
            .map(|mime_type| json!(format!("data:{};base64,{}", mime_type, base64::encode(&buffer))))
    })
}
