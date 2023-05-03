use crate::behaviour::entity::random_u64::RandomU64Factory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_random::RandomU64;
use crate::model_random::BEHAVIOUR_RANDOM_U64;
use crate::model_random::ENTITY_TYPE_RANDOM_U64;
use crate::model_result::ResultNumberU64;
use crate::model_result::ResultNumberU64Properties::RESULT;
use crate::model_runtime::Action;
use crate::model_runtime::ActionProperties::TRIGGER;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn random_u64_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_RANDOM_U64.clone())
        .property_with_default(&TRIGGER)
        .property_with_default(&RESULT)
        .build();
    let random_u64 = RandomU64::from(reactive_instance.clone());

    let next;
    {
        let factory = RandomU64Factory::new(BEHAVIOUR_RANDOM_U64.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        random_u64.trigger();
        let previous = random_u64.result().unwrap();
        random_u64.trigger();
        next = random_u64.result().unwrap();
        assert_ne!(previous, next);
    }

    random_u64.trigger();
    let should_not_have_changed = random_u64.result().unwrap();
    assert_eq!(next, should_not_have_changed);
}
