use std::path::PathBuf;

use inexor_rgf_model_binary::BinaryDataUrl;
use inexor_rgf_model_binary::File;
use serde_json::json;

use crate::behaviour::component::load_binary_data::LoadBinaryDataFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_binary::Action;
use crate::model_binary::ActionProperties::TRIGGER;
use crate::model_binary::BinaryData;
use crate::model_binary::BinaryDataProperties::DATA_URL;
use crate::model_binary::FileProperties::FILENAME;
use crate::model_binary::LoadBinaryData;
use crate::model_binary::BEHAVIOUR_LOAD_BINARY_DATA;
use crate::model_binary::ENTITY_TYPE_LOAD_BINARY_DATA;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn rx_load_binary_data_test() {
    let mut load_png_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    load_png_file_path.push("../../../../docs/images/inexor_2.png");
    load_png_file_path = load_png_file_path.canonicalize().unwrap();

    let mut type_definition_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    type_definition_path.push("../../assets/types/components/load_binary_data.json");
    type_definition_path = type_definition_path.canonicalize().unwrap();

    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_LOAD_BINARY_DATA.clone())
        .property(DATA_URL, json!(""))
        .property(TRIGGER, json!(false))
        .property(FILENAME, json!(load_png_file_path.to_str().unwrap()))
        .build();

    let load_binary_data = LoadBinaryData::from(reactive_instance.clone());

    load_binary_data.trigger();
    let data_url = load_binary_data.get_data_url().unwrap();
    assert_eq!(0, data_url.len());

    {
        let factory = LoadBinaryDataFactory::new(BEHAVIOUR_LOAD_BINARY_DATA.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        println!("{}", load_png_file_path.display());
        load_binary_data.trigger();
        let data_url = load_binary_data.get_data_url().unwrap();
        println!("{}...[size: {} bytes, {} chars]", data_url.split_at(50).0, data_url.len(), data_url.chars().count());
        assert!(data_url.len() > 0);
        assert_eq!("image", load_binary_data.mime_type().unwrap());
        assert_eq!("png", load_binary_data.subtype().unwrap());

        println!("{}", type_definition_path.display());
        load_binary_data.filename(type_definition_path.to_str().unwrap());
        let data_url = load_binary_data.get_data_url().unwrap();
        println!("{}...[size: {} bytes, {} chars]", data_url.split_at(50).0, data_url.len(), data_url.chars().count());
        assert!(data_url.len() > 0);
        assert_eq!("application", load_binary_data.mime_type().unwrap());
        assert_eq!("json", load_binary_data.subtype().unwrap());
    }
}
