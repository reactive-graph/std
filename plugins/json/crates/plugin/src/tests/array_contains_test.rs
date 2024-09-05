use reactive_graph_model_json::ArrayContains;
use reactive_graph_model_json::ArrayContainsProperties::ARRAY;
use reactive_graph_model_json::ArrayContainsProperties::SEARCH;
use reactive_graph_model_json::BEHAVIOUR_ARRAY_CONTAINS;
use reactive_graph_model_json::ENTITY_TYPE_ARRAY_CONTAINS;
use reactive_graph_model_result::ResultBoolean;
use reactive_graph_model_result::ResultBooleanProperties::RESULT;
use serde_json::json;

use crate::behaviour::entity::array_contains::ArrayContainsFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn array_contains_test() {
    let array_2_elements = json!(vec![1, 2]);
    let array_3_elements = json!(vec![1, 2, 3]);

    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_ARRAY_CONTAINS.clone())
        .property_with_default(&RESULT)
        .property_with_default(&SEARCH)
        .property_with_default(&ARRAY)
        .build();
    let array_contains = ArrayContains::from(reactive_instance.clone());

    // No behaviour => no change
    array_contains.array(array_2_elements.as_array().cloned().unwrap());
    array_contains.search(json!(2));
    assert!(!array_contains.result().unwrap());

    // With behaviour => array_contains.array changes array_contains.result
    {
        let factory = ArrayContainsFactory::new(BEHAVIOUR_ARRAY_CONTAINS.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        array_contains.search(json!(0));
        assert!(!array_contains.result().unwrap());
        array_contains.search(json!(1));
        assert!(array_contains.result().unwrap());
        array_contains.search(json!(2));
        assert!(array_contains.result().unwrap());
        array_contains.search(json!(3));
        assert!(!array_contains.result().unwrap());
        array_contains.search(json!(4));
        assert!(!array_contains.result().unwrap());

        array_contains.array(array_3_elements.as_array().cloned().unwrap());
        array_contains.search(json!(0));
        assert!(!array_contains.result().unwrap());
        array_contains.search(json!(1));
        assert!(array_contains.result().unwrap());
        array_contains.search(json!(2));
        assert!(array_contains.result().unwrap());
        array_contains.search(json!(3));
        assert!(array_contains.result().unwrap());
        array_contains.search(json!(4));
        assert!(!array_contains.result().unwrap());
    }
}
