use crate::behaviour::entity::random_u64_range::RandomU64RangeFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use reactive_graph_model_random::RandomU64Range;
use reactive_graph_model_random::RangeU64;
use reactive_graph_model_random::RangeU64Properties::HIGH;
use reactive_graph_model_random::RangeU64Properties::LOW;
use reactive_graph_model_random::BEHAVIOUR_RANDOM_U64_RANGE;
use reactive_graph_model_random::ENTITY_TYPE_RANDOM_U64_RANGE;
use reactive_graph_model_result::ResultNumberU64;
use reactive_graph_model_result::ResultNumberU64Properties::RESULT;
use reactive_graph_runtime_model::Action;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;

#[test]
fn random_u64_range_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_RANDOM_U64_RANGE.clone())
        .property_with_default(&TRIGGER)
        .property_with_default(&LOW)
        .property_with_default(&HIGH)
        .property_with_default(&RESULT)
        .build();
    let random_u64_range = RandomU64Range::from(reactive_instance.clone());

    let next;
    {
        let factory = RandomU64RangeFactory::new(BEHAVIOUR_RANDOM_U64_RANGE.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        random_u64_range.low(10);
        random_u64_range.high(50);
        for _ in 0..1000 {
            random_u64_range.trigger();
            let result = random_u64_range.result().unwrap();
            assert!(result >= 10 && result < 50);
        }
        random_u64_range.low(0);
        random_u64_range.high(100);
        for _ in 0..1000 {
            random_u64_range.trigger();
            let result = random_u64_range.result().unwrap();
            assert!(result < 100);
        }
        random_u64_range.trigger();
        next = random_u64_range.result().unwrap();
    }

    random_u64_range.trigger();
    let should_not_have_changed = random_u64_range.result().unwrap();
    assert_eq!(next, should_not_have_changed);
}
