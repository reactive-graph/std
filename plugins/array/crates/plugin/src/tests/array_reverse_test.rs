use reactive_graph_std_array_model::ArrayReverse;
use reactive_graph_std_array_model::ArrayReverseProperties::ARRAY;
use reactive_graph_std_array_model::BEHAVIOUR_ARRAY_REVERSE;
use reactive_graph_std_array_model::ENTITY_TYPE_ARRAY_REVERSE;
use reactive_graph_std_result_model::ResultArray;
use reactive_graph_std_result_model::ResultArrayProperties::RESULT;
use serde_json::json;

use crate::behaviour::entity::array_reverse::ArrayReverseFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn rx_array_reverse_test() {
    let empty_array = json!([]);
    let array = json!(vec![1, 2, 3]);
    let reversed_array = json!(vec![3, 2, 1]);

    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_ARRAY_REVERSE.clone())
        .property_with_default(&RESULT)
        .property_with_default(&ARRAY)
        .build();
    let array_reverse = ArrayReverse::from(reactive_instance.clone());

    // No behaviour => no change
    array_reverse.array(array.as_array().cloned().unwrap());
    let result = array_reverse.result().unwrap();
    assert_eq!(empty_array.as_array().unwrap(), &result);
    {
        let factory = ArrayReverseFactory::new(BEHAVIOUR_ARRAY_REVERSE.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        // With behaviour => array_reverse.array changes array_reverse.result
        let cloned_array = array.as_array().cloned().unwrap();
        array_reverse.array(cloned_array);
        let result = array_reverse.result().unwrap();
        assert_eq!(reversed_array.as_array().unwrap(), &result);
    }
}
