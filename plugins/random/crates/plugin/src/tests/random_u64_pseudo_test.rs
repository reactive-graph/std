use crate::behaviour::entity::random_u64_pseudo::RandomU64PseudoFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_random::PseudoNumberGenerator;
use crate::model_random::PseudoNumberGeneratorProperties::SEED;
use crate::model_random::RandomU64Pseudo;
use crate::model_random::BEHAVIOUR_RANDOM_U64_PSEUDO;
use crate::model_random::ENTITY_TYPE_RANDOM_U64_PSEUDO;
use crate::model_result::ResultNumberU64;
use crate::model_result::ResultNumberU64Properties::RESULT;
use crate::model_trigger::Action;
use crate::model_trigger::ActionProperties::TRIGGER;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn random_u64_pseudo_test() {
    let reactive_instance = ReactiveEntityInstanceBuilder::new(ENTITY_TYPE_RANDOM_U64_PSEUDO.clone())
        .property_with_default(&TRIGGER)
        .property_with_default(&SEED)
        .property_with_default(&RESULT)
        .build();
    let random_u64_pseudo = RandomU64Pseudo::from(reactive_instance.clone());

    let factory = RandomU64PseudoFactory::new(BEHAVIOUR_RANDOM_U64_PSEUDO.clone());

    // Round 1: Reconnect should produce the same pseudo random numbers
    let first;
    let second;
    random_u64_pseudo.seed(10);
    {
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());
        random_u64_pseudo.trigger();
        first = random_u64_pseudo.result().unwrap();
        random_u64_pseudo.trigger();
        second = random_u64_pseudo.result().unwrap();
        println!("{first} {second}");
        behaviour
            .transition(BehaviourState::Ready)
            .and_then(|_| behaviour.transition(BehaviourState::Connected))
            .expect("Failed to reconnect behaviour");
        random_u64_pseudo.trigger();
        assert_eq!(first, random_u64_pseudo.result().unwrap());
        random_u64_pseudo.trigger();
        assert_eq!(second, random_u64_pseudo.result().unwrap());
    }

    // Round 2: Another seed should produce other pseudo random numbers as in round 1 but reconnecting with the same seed should produce the same pseudo numbers
    let third;
    let forth;
    random_u64_pseudo.seed(11);
    {
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());
        random_u64_pseudo.trigger();
        third = random_u64_pseudo.result().unwrap();
        assert_ne!(first, third);
        random_u64_pseudo.trigger();
        forth = random_u64_pseudo.result().unwrap();
        assert_ne!(second, forth);
        println!("{third} {forth}");
        behaviour
            .transition(BehaviourState::Ready)
            .and_then(|_| behaviour.transition(BehaviourState::Connected))
            .expect("Failed to reconnect behaviour");
        random_u64_pseudo.trigger();
        assert_eq!(third, random_u64_pseudo.result().unwrap());
        random_u64_pseudo.trigger();
        assert_eq!(forth, random_u64_pseudo.result().unwrap());
    }

    // Round 3: Use the same seed as in round 1. This should produce the same pseudo random numbers than in round 1.
    random_u64_pseudo.seed(10);
    {
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());
        random_u64_pseudo.trigger();
        let fifth = random_u64_pseudo.result().unwrap();
        assert_eq!(first, fifth);
        random_u64_pseudo.trigger();
        let sixth = random_u64_pseudo.result().unwrap();
        assert_eq!(second, sixth);
        println!("{fifth} {sixth}");
    }
}
