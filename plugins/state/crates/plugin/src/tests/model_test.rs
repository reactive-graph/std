use serde_json::json;

use crate::behaviour::component::StateFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_state::SetStateBoolean;
use crate::model_state::StateBoolean;
use crate::model_state::StateProperties::SET_STATE;
use crate::model_state::StateProperties::STATE;
use crate::model_state::COMPONENT_STATE;
use crate::model_state::ENTITY_TYPE_STATE_BOOLEAN;
use crate::model_value::GetValueBoolean;
use crate::model_value::ValueProperties::VALUE;
use crate::model_value::COMPONENT_VALUE;
use crate::reactive::BehaviourFactory;
use reactive_graph_graph::BehaviourTypeId;
use reactive_graph_graph::NamespacedType;

#[test]
fn state_bool_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_STATE_BOOLEAN.clone())
        .property(VALUE, json!(false))
        .property(STATE, json!(false))
        .property(SET_STATE, json!(false))
        .component(COMPONENT_VALUE.clone())
        .component(COMPONENT_STATE.clone())
        .build();
    let state = StateBoolean::from(reactive_instance.clone());
    assert!(!state.value().unwrap());
    // No change because no behaviour
    state.set_state(true);
    assert!(!state.value().unwrap());
    {
        let behaviour_ty = BehaviourTypeId::from(NamespacedType::from(&ENTITY_TYPE_STATE_BOOLEAN.clone()));
        let state_boolean_factory = StateFactory::new(behaviour_ty);
        let behaviour = state_boolean_factory.create(reactive_instance.clone());
        assert!(behaviour.is_ok());
        // The behaviour changes the value set_state -> state -> value
        state.set_state(true);
        assert!(state.value().unwrap());
    }
    // No change because no behaviour
    state.set_state(false);
    assert!(state.value().unwrap());
}
