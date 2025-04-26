use reactive_graph_std_array_model::ArrayLength;
use reactive_graph_std_array_model::ArrayLengthProperties::ARRAY;
use reactive_graph_std_array_model::ArrayLengthProperties::LENGTH;
use reactive_graph_std_array_model::BEHAVIOUR_ARRAY_LENGTH;
use reactive_graph_std_array_model::ENTITY_TYPE_ARRAY_LENGTH;
use serde_json::json;

use crate::behaviour::entity::array_length::ArrayLengthFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn array_length_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_ARRAY_LENGTH.clone())
        .property_with_default(&LENGTH)
        .property_with_default(&ARRAY)
        .build();
    let array_length = ArrayLength::from(reactive_instance.clone());

    {
        let factory = ArrayLengthFactory::new(BEHAVIOUR_ARRAY_LENGTH.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        array_length.array(vec![]);
        assert_eq!(0, array_length.length().unwrap());

        array_length.array(vec![json!(1)]);
        assert_eq!(1, array_length.length().unwrap());

        array_length.array(vec![json!(1), json!(2)]);
        assert_eq!(2, array_length.length().unwrap());

        array_length.array(vec![json!(1), json!(2), json!(3)]);
        assert_eq!(3, array_length.length().unwrap());
    }
}
