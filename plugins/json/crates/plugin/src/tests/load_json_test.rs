use std::path::PathBuf;

use serde_json::json;
use serde_json::to_string_pretty;

use crate::behaviour::component::load_json::LoadJsonFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use reactive_graph_model_file::FileProperties::FILENAME;
use reactive_graph_model_json::LoadJson;
use reactive_graph_model_json::BEHAVIOUR_LOAD_JSON;
use reactive_graph_model_json::ENTITY_TYPE_LOAD_JSON;
use reactive_graph_model_result::ResultAny;
use reactive_graph_model_result::ResultObjectProperties::RESULT;
use reactive_graph_runtime_model::Action;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;

#[test]
fn rx_load_json_test() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("types/components/load_json.json");
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
