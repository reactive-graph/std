use std::fs::File;
use std::io::Read;
use std::path::Path;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use mime_guess::from_path;
use reactive_graph_behaviour_model_api::behaviour_validator;
use reactive_graph_behaviour_model_api::prelude::*;
use reactive_graph_behaviour_model_impl::entity_behaviour;
use reactive_graph_graph::prelude::*;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use serde_json::Value;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_model_binary::BinaryDataProperties::DATA_URL;
use reactive_graph_model_file::FileProperties::FILENAME;

entity_behaviour!(
    LoadBinaryData,
    LoadBinaryDataFactory,
    LoadBinaryDataFsm,
    LoadBinaryDataBehaviourTransitions,
    LoadBinaryDataValidator
);

behaviour_validator!(LoadBinaryDataValidator, Uuid, ReactiveEntity, TRIGGER.as_ref(), FILENAME.as_ref(), DATA_URL.as_ref());

impl BehaviourInit<Uuid, ReactiveEntity> for LoadBinaryDataBehaviourTransitions {
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        if self.reactive_instance.as_bool(TRIGGER).unwrap_or(false) {
            if let Some(value) = self.reactive_instance.as_string(FILENAME).and_then(load_binary_data) {
                self.reactive_instance.set(DATA_URL, value);
            }
        }
        Ok(())
    }
}

impl BehaviourConnect<Uuid, ReactiveEntity> for LoadBinaryDataBehaviourTransitions {
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(TRIGGER.as_ref(), move |trigger: &Value| {
            if !trigger.as_bool().unwrap_or(false) {
                // Invalid type
                return;
            }
            if let Some(value) = reactive_instance.as_string(FILENAME).and_then(load_binary_data) {
                reactive_instance.set(DATA_URL, value);
            }
        });
        let reactive_instance = self.reactive_instance.clone();
        self.property_observers.observe_with_handle(FILENAME.as_ref(), move |filename: &Value| {
            if let Some(value) = filename.as_str().map(String::from).and_then(load_binary_data) {
                reactive_instance.set(DATA_URL, value);
            }
        });
        Ok(())
    }
}

impl BehaviourShutdown<Uuid, ReactiveEntity> for LoadBinaryDataBehaviourTransitions {}
impl BehaviourTransitions<Uuid, ReactiveEntity> for LoadBinaryDataBehaviourTransitions {}

fn load_binary_data(filename: String) -> Option<Value> {
    let filename = shellexpand::tilde(&filename);
    let path = Path::new(filename.as_ref());
    File::open(path).ok().and_then(|mut file| {
        let mut buffer = Vec::new();
        let _ = file.read_to_end(&mut buffer);
        infer::get(&buffer)
            .map(|kind| kind.mime_type().to_string())
            .or_else(|| from_path(path).first().map(|x| x.to_string()))
            .map(|mime_type| json!(format!("data:{};base64,{}", mime_type, STANDARD.encode(&buffer))))
    })
}
