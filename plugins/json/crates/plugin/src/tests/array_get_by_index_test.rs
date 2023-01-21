use serde_json::json;

use crate::behaviour::entity::array_get_by_index::ArrayGetByIndexFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_json::ArrayGetByIndex;
use crate::model_json::ArrayGetByIndexProperties::ARRAY;
use crate::model_json::ArrayGetByIndexProperties::INDEX;
use crate::model_json::BEHAVIOUR_ARRAY_GET_BY_INDEX;
use crate::model_json::ENTITY_TYPE_ARRAY_GET_BY_INDEX;
use crate::model_result::ResultAny;
use crate::model_result::ResultAnyProperties::RESULT;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn array_get_by_index_test() {
    let array_3_elements = json!(vec![1, 2, 3]);

    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_ARRAY_GET_BY_INDEX.clone())
        .property_with_default(&RESULT)
        .property_with_default(&INDEX)
        .property_with_default(&ARRAY)
        .build();
    let array_get_by_index = ArrayGetByIndex::from(reactive_instance.clone());

    {
        let factory = ArrayGetByIndexFactory::new(BEHAVIOUR_ARRAY_GET_BY_INDEX.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        array_get_by_index.array(array_3_elements.as_array().cloned().unwrap());
        array_get_by_index.index(0);
        assert_eq!(json!(1), array_get_by_index.result().unwrap());
        array_get_by_index.index(1);
        assert_eq!(json!(2), array_get_by_index.result().unwrap());
        array_get_by_index.index(2);
        assert_eq!(json!(3), array_get_by_index.result().unwrap());
    }
}
