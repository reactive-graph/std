use serde_json::json;

use crate::behaviour::entity::trigger::TriggerFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_model_runtime::COMPONENT_ACTION;
use reactive_graph_runtime_model::Action;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use reactive_graph_std_logical_model::BEHAVIOUR_TRIGGER;
use reactive_graph_std_logical_model::ENTITY_TYPE_NAME_TRIGGER;
use reactive_graph_std_logical_model::ENTITY_TYPE_TRIGGER;
use reactive_graph_std_logical_model::NAMESPACE_LOGICAL;
use reactive_graph_std_logical_model::Trigger;
use reactive_graph_std_logical_model::TriggerProperties::PAYLOAD;
use reactive_graph_std_result_model::ResultAny;
use reactive_graph_std_result_model::ResultAnyProperties::RESULT;

#[test]
fn trigger_test() {
    let entity_ty = ENTITY_TYPE_TRIGGER.clone();
    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(TRIGGER, json!(false))
        .property(RESULT, json!(false))
        .property(PAYLOAD, json!(false))
        .component(COMPONENT_ACTION.clone())
        .build();

    let trigger = Trigger::from(reactive_instance.clone());

    assert_eq!(NAMESPACE_LOGICAL, trigger.namespace().as_str());
    assert_eq!(ENTITY_TYPE_NAME_TRIGGER, trigger.type_name().as_str());

    {
        let behaviour_ty = BEHAVIOUR_TRIGGER.clone();
        let factory = TriggerFactory::new(behaviour_ty.clone());
        let behaviour = factory.create(reactive_instance.clone());
        assert!(behaviour.is_ok());

        trigger.payload(json!("Hello"));
        trigger.trigger();
        assert_eq!("Hello", trigger.result().unwrap().as_str().unwrap());
        trigger.trigger();
        assert_eq!("Hello", trigger.result().unwrap().as_str().unwrap());
        trigger.payload(json!("World"));
        assert_eq!("Hello", trigger.result().unwrap().as_str().unwrap());
        trigger.trigger();
        assert_eq!("World", trigger.result().unwrap().as_str().unwrap());
    }
    // The behaviour has been dropped
    trigger.payload(json!("Inexor"));
    trigger.trigger();
    assert_eq!("World", trigger.result().unwrap().as_str().unwrap());
}
