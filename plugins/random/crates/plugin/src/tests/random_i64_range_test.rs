use crate::behaviour::entity::random_i64_range::RandomI64RangeFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use inexor_rgf_model_random::RandomI64Range;
use inexor_rgf_model_random::RangeI64;
use inexor_rgf_model_random::RangeI64Properties::HIGH;
use inexor_rgf_model_random::RangeI64Properties::LOW;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_I64_RANGE;
use inexor_rgf_model_random::ENTITY_TYPE_RANDOM_I64_RANGE;
use inexor_rgf_model_result::ResultNumberI64;
use inexor_rgf_model_result::ResultNumberI64Properties::RESULT;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_model_runtime::ActionProperties::TRIGGER;

#[test]
fn random_i64_range_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_RANDOM_I64_RANGE.clone())
        .property_with_default(&TRIGGER)
        .property_with_default(&LOW)
        .property_with_default(&HIGH)
        .property_with_default(&RESULT)
        .build();
    let random_i64_range = RandomI64Range::from(reactive_instance.clone());

    let next;
    {
        let factory = RandomI64RangeFactory::new(BEHAVIOUR_RANDOM_I64_RANGE.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        random_i64_range.low(-50);
        random_i64_range.high(50);
        for _ in 0..1000 {
            random_i64_range.trigger();
            let result = random_i64_range.result().unwrap();
            assert!(result >= -50 && result < 50);
        }
        random_i64_range.low(-100);
        random_i64_range.high(100);
        for _ in 0..1000 {
            random_i64_range.trigger();
            let result = random_i64_range.result().unwrap();
            assert!(result >= -100 && result < 100);
        }
        random_i64_range.trigger();
        next = random_i64_range.result().unwrap();
    }

    random_i64_range.trigger();
    let should_not_have_changed = random_i64_range.result().unwrap();
    assert_eq!(next, should_not_have_changed);
}
