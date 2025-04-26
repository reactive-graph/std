use crate::behaviour::entity::random_uuid::RandomUuidFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use reactive_graph_runtime_model::Action;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use reactive_graph_std_random_model::BEHAVIOUR_RANDOM_UUID;
use reactive_graph_std_random_model::ENTITY_TYPE_RANDOM_UUID;
use reactive_graph_std_random_model::RandomUuid;
use reactive_graph_std_result_model::ResultString;
use reactive_graph_std_result_model::ResultStringProperties::RESULT;

#[test]
fn random_uuid_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_RANDOM_UUID.clone())
        .property_with_default(&TRIGGER)
        .property_with_default(&RESULT)
        .build();
    let random_uuid = RandomUuid::from(reactive_instance.clone());

    let next;
    {
        let factory = RandomUuidFactory::new(BEHAVIOUR_RANDOM_UUID.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        random_uuid.trigger();
        let previous = random_uuid.result().unwrap();
        random_uuid.trigger();
        next = random_uuid.result().unwrap();
        assert_ne!(previous, next);
    }

    random_uuid.trigger();
    let should_not_have_changed = random_uuid.result().unwrap();
    assert_eq!(next, should_not_have_changed);
}
