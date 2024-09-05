use serde_json::json;

use crate::behaviour::entity::toggle::ToggleFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_model_logical::Toggle;
use reactive_graph_model_logical::BEHAVIOUR_TOGGLE;
use reactive_graph_model_logical::ENTITY_TYPE_NAME_TOGGLE;
use reactive_graph_model_logical::ENTITY_TYPE_TOGGLE;
use reactive_graph_model_logical::NAMESPACE_LOGICAL;
use reactive_graph_model_result::ResultBoolean;
use reactive_graph_model_result::ResultBooleanProperties::RESULT;
use reactive_graph_model_runtime::COMPONENT_ACTION;
use reactive_graph_runtime_model::Action;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;

#[test]
fn toggle_test() {
    let entity_ty = ENTITY_TYPE_TOGGLE.clone();
    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(TRIGGER, json!(false))
        .property(RESULT, json!(false))
        .component(COMPONENT_ACTION.clone())
        .build();

    let toggle = Toggle::from(reactive_instance.clone());

    assert_eq!(NAMESPACE_LOGICAL, toggle.namespace().as_str());
    assert_eq!(ENTITY_TYPE_NAME_TOGGLE, toggle.type_name().as_str());

    {
        let behaviour_ty = BEHAVIOUR_TOGGLE.clone();
        let factory = ToggleFactory::new(behaviour_ty.clone());
        let behaviour = factory.create(reactive_instance.clone());
        assert!(behaviour.is_ok());

        toggle.trigger();
        assert_eq!(true, toggle.result().unwrap());
        toggle.trigger();
        assert_eq!(false, toggle.result().unwrap());
        toggle.trigger();
        assert_eq!(true, toggle.result().unwrap());
        toggle.trigger();
        assert_eq!(false, toggle.result().unwrap());
    }
    // The behaviour has been dropped
    toggle.trigger();
    assert_eq!(false, toggle.result().unwrap());
}
