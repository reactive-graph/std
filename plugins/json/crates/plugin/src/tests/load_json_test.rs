use std::path::PathBuf;

use serde_json::json;
use serde_json::to_string_pretty;

use crate::behaviour::component::load_json::LoadJsonFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_file::FileProperties::FILENAME;
use crate::model_json::LoadJson;
use crate::model_json::BEHAVIOUR_LOAD_JSON;
use crate::model_json::ENTITY_TYPE_LOAD_JSON;
use crate::model_result::ResultAny;
use crate::model_result::ResultObjectProperties::RESULT;
use crate::model_trigger::Action;
use crate::model_trigger::ActionProperties::TRIGGER;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn rx_load_json_test() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../types/components/load_json.json");
    path = path.canonicalize().unwrap();

    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_LOAD_JSON.clone())
        .property(RESULT, json!({}))
        .property_with_default(&TRIGGER)
        .property(FILENAME, json!(path.to_str().unwrap()))
        .build();

    let load_json = LoadJson::from(reactive_instance.clone());

    load_json.trigger();
    let json = load_json.result().unwrap();
    let json_pretty_str = to_string_pretty(&json).unwrap();
    assert_eq!(2, json_pretty_str.len());

    {
        let factory = LoadJsonFactory::new(BEHAVIOUR_LOAD_JSON.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        load_json.trigger();
        let json = load_json.result().unwrap();
        let json_pretty_str = to_string_pretty(&json).unwrap();
        assert!(json_pretty_str.len() > 2);
        println!("{}", json_pretty_str);
    }
}
