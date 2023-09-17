use crate::behaviour::entity::random_f64::RandomF64Factory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use inexor_rgf_model_random::RandomF64;
use inexor_rgf_model_random::BEHAVIOUR_RANDOM_F64;
use inexor_rgf_model_random::ENTITY_TYPE_RANDOM_F64;
use inexor_rgf_model_result::ResultNumberF64;
use inexor_rgf_model_result::ResultNumberF64Properties::RESULT;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_model_runtime::ActionProperties::TRIGGER;

#[test]
fn random_f64_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_RANDOM_F64.clone())
        .property_with_default(&TRIGGER)
        .property_with_default(&RESULT)
        .build();
    let random_f64 = RandomF64::from(reactive_instance.clone());

    let next;
    {
        let factory = RandomF64Factory::new(BEHAVIOUR_RANDOM_F64.clone());
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        random_f64.trigger();
        let previous = random_f64.result().unwrap();
        random_f64.trigger();
        next = random_f64.result().unwrap();
        assert_ne!(previous, next);
    }

    random_f64.trigger();
    let should_not_have_changed = random_f64.result().unwrap();
    assert_eq!(next, should_not_have_changed);
}
