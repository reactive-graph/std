use crate::behaviour::entity::string_gate::StringGateFactory;
use crate::behaviour::entity::string_gate::STRING_GATES;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::BehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model_result::ResultString;
use crate::model_result::ResultStringProperties::RESULT;
use crate::model_string::StringGate;
use crate::model_string::StringGateProperties::LHS;
use crate::model_string::StringGateProperties::RHS;
use crate::model_string::COMPONENT_STRING_GATE;
use crate::model_string::NAMESPACE_STRING;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn rx_concat_test() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_STRING, "concat");
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_STRING, "concat");

    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty)
        .property_with_default(&LHS)
        .property_with_default(&RHS)
        .property_with_default(&RESULT)
        .component(COMPONENT_STRING_GATE.clone())
        .build();
    let concat = StringGate::from(reactive_instance.clone());

    // No behaviour -> no change
    concat.lhs("Hello, ");
    concat.rhs("World");
    assert_eq!("", concat.result().unwrap());

    // With behaviour
    {
        let f = STRING_GATES.get(&behaviour_ty).cloned().expect("Failed to find behaviour function");
        let factory = StringGateFactory::new(behaviour_ty, f);
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        concat.lhs("Hello, ");
        concat.rhs("World");
        assert_eq!("Hello, World", concat.result().unwrap());

        concat.lhs("Servus, ");
        assert_eq!("Servus, World", concat.result().unwrap());

        concat.rhs("Inexor");
        assert_eq!("Servus, Inexor", concat.result().unwrap());
    }

    // No behaviour -> no change
    concat.lhs("Hello, ");
    concat.rhs("World");
    assert_eq!("Servus, Inexor", concat.result().unwrap());
}
