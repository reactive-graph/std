use crate::behaviour::entity::random_uuid::RandomUuidFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_random::RandomUuid;
use crate::model_random::RandomUuidProperties::RESULT;
use crate::model_random::RandomUuidProperties::TRIGGER;
use crate::model_random::BEHAVIOUR_RANDOM_UUID;
use crate::model_random::ENTITY_TYPE_RANDOM_UUID;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

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
