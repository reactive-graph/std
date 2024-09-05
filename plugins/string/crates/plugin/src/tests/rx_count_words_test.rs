use crate::behaviour::entity::string_string_number_gate::StringStringNumberGateFactory;
use crate::behaviour::entity::string_string_number_gate::STRING_STRING_NUMBER_GATES;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_string::StringStringNumberGate;
use crate::model_string::StringStringNumberGateProperties::LHS;
use crate::model_string::StringStringNumberGateProperties::RHS;
use crate::model_string::COMPONENT_STRING_STRING_NUMBER_GATE;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use reactive_graph_graph::BehaviourTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_model_result::ResultNumberU64;
use reactive_graph_model_result::ResultNumberU64Properties::RESULT;
use reactive_graph_model_string::NAMESPACE_STRING;

#[test]
fn rx_count_words_test() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_STRING, "count_words");
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_STRING, "count_words");

    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty)
        .property_with_default(&LHS)
        .property_with_default(&RHS)
        .property_with_default(&RESULT)
        .component(COMPONENT_STRING_STRING_NUMBER_GATE.clone())
        .build();
    let count_words = StringStringNumberGate::from(reactive_instance.clone());

    // No behaviour -> no change
    count_words.lhs("Hello World");
    count_words.rhs("");
    assert_eq!(0, count_words.result().unwrap());

    // With behaviour
    {
        let f = STRING_STRING_NUMBER_GATES
            .get(&behaviour_ty)
            .cloned()
            .expect("Failed to find behaviour function");
        let factory = StringStringNumberGateFactory::new(behaviour_ty, f);
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        count_words.lhs("Gravity - can cross dimensions!");
        assert_eq!(4, count_words.result().unwrap());

        count_words.lhs("Donaudampfschifffahrtselektrizitätenhauptbetriebswerkbauunterbeamtengesellschaft");
        assert_eq!(1, count_words.result().unwrap());

        count_words.lhs("GravityCanCrossDimensions");
        assert_eq!(4, count_words.result().unwrap());

        count_words.lhs("Cafe\u{0301}-del-Mar-andBossaNova1");
        count_words.rhs("-");
        assert_eq!(4, count_words.result().unwrap());
    }

    // No behaviour -> no change
    count_words.lhs("Gravity");
    assert_eq!(4, count_words.result().unwrap());
}
