use serde_json::json;

use crate::behaviour::entity::toggle::ToggleFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::NamespacedTypeGetter;
use crate::model_logical::ActionProperties::RESULT;
use crate::model_logical::ActionProperties::TRIGGER;
use crate::model_logical::Toggle;
use crate::model_logical::BEHAVIOUR_TOGGLE;
use crate::model_logical::COMPONENT_ACTION;
use crate::model_logical::ENTITY_TYPE_NAME_TOGGLE;
use crate::model_logical::ENTITY_TYPE_TOGGLE;
use crate::model_logical::NAMESPACE_LOGICAL;
use crate::reactive::BehaviourFactory;

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

        toggle.trigger(true);
        assert_eq!(true, toggle.result().unwrap());
        toggle.trigger(true);
        assert_eq!(false, toggle.result().unwrap());
        toggle.trigger(true);
        assert_eq!(true, toggle.result().unwrap());
        toggle.trigger(false);
        assert_eq!(true, toggle.result().unwrap());
        toggle.trigger(true);
        assert_eq!(false, toggle.result().unwrap());
        toggle.trigger(false);
        assert_eq!(false, toggle.result().unwrap());
    }
    // The behaviour has been dropped
    toggle.trigger(true);
    assert_eq!(false, toggle.result().unwrap());
    toggle.trigger(false);
    assert_eq!(false, toggle.result().unwrap());
}
