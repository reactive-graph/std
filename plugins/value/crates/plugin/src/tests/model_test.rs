use serde_json::json;

use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_value::DataValueBoolean;
use crate::model_value::ValueBoolean;
use crate::model_value::ValueProperties::VALUE;
use crate::model_value::COMPONENT_VALUE;
use crate::model_value::ENTITY_TYPE_VALUE_BOOLEAN;

#[test]
fn value_bool_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_VALUE_BOOLEAN.clone())
        .property(VALUE, json!(false))
        .component(COMPONENT_VALUE.clone())
        .build();
    let entity = ValueBoolean::from(reactive_instance);
    assert!(!entity.get_value().unwrap());
    entity.set_value(true);
    assert!(entity.get_value().unwrap());
}
