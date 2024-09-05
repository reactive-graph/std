use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use serde_json::json;
use uuid::Uuid;

use crate::behaviour::component::StateFactory;
use crate::behaviour::component::STATE_BEHAVIOURS;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_state::StateProperties;
use crate::model_value::ValueProperties;
use crate::reactive::BehaviourFactory;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::NamespacedType;
use reactive_graph_graph::PropertyInstanceSetter;

#[test]
fn state_behaviour_test() {
    for behaviour_ty in STATE_BEHAVIOURS.iter() {
        let namespaced_ty = NamespacedType::from(behaviour_ty);
        let entity_instance = ReactiveEntityInstanceBuilder::new(EntityTypeId::from(namespaced_ty))
            .property(StateProperties::STATE, json!(0))
            .property(StateProperties::SET_STATE, json!(0))
            .property(ValueProperties::VALUE, json!(0))
            .build();

        // Create atomic bool state
        let has_changed = Arc::new(AtomicBool::new(false));

        // Move atomic into closure
        let c_has_changed = has_changed.clone();

        // Observe the target property "state" for changes
        entity_instance
            .properties
            .get(StateProperties::STATE.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |_v| {
                    // If the closure has been called, the previous state has actually changed
                    c_has_changed.store(true, Ordering::Relaxed);
                },
                Uuid::new_v4().as_u128(),
            );

        // No change yet
        assert!(!has_changed.load(Ordering::Relaxed));

        {
            // Create behaviour
            let factory = StateFactory::new(behaviour_ty.clone());
            let behaviour = factory.create(entity_instance.clone());
            assert!(behaviour.is_ok());

            // Set state without changing its initial value
            entity_instance.set(StateProperties::SET_STATE, json!(0));

            // No change because the value hasn't changed
            assert!(!has_changed.load(Ordering::Relaxed));

            // Actually change the value
            entity_instance.set(StateProperties::SET_STATE, json!(1));

            // Changed because the value has changed!
            assert!(has_changed.load(Ordering::Relaxed));

            // Reset
            has_changed.store(false, Ordering::Relaxed);

            // Set state without changing its current value
            entity_instance.set(StateProperties::SET_STATE, json!(1));

            // No change because the value hasn't changed
            assert!(!has_changed.load(Ordering::Relaxed));

            // Change the value again
            entity_instance.set(StateProperties::SET_STATE, json!(0));

            // Changed again!
            assert!(has_changed.load(Ordering::Relaxed));
        }
        // Scope drop -> Behaviour destructs automatically and the internal wiring is removed

        // Reset
        has_changed.store(false, Ordering::Relaxed);

        // Change the value
        entity_instance.set(StateProperties::SET_STATE, json!(2));

        // No change because the behaviour doesn't exist anymore and the reactive behaviour for "set_state" has been destructed!
        assert!(!has_changed.load(Ordering::Relaxed));
    }
}
