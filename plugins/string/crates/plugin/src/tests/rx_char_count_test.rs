use crate::behaviour::entity::string_number_operation::STRING_NUMBER_OPERATIONS;
use crate::behaviour::entity::string_number_operation::StringNumberOperationFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_string::COMPONENT_STRING_NUMBER_OPERATION;
use crate::model_string::StringNumberOperation;
use crate::model_string::StringNumberOperationProperties::LHS;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use reactive_graph_graph::BehaviourTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_std_result_model::ResultNumberU64;
use reactive_graph_std_result_model::ResultNumberU64Properties::RESULT;
use reactive_graph_std_string_model::NAMESPACE_STRING;

#[test]
fn rx_char_count_test() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_STRING, "char_count");
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_STRING, "char_count");

    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty)
        .property_with_default(&LHS)
        .property_with_default(&RESULT)
        .component(COMPONENT_STRING_NUMBER_OPERATION.clone())
        .build();
    let char_count = StringNumberOperation::from(reactive_instance.clone());

    // No behaviour -> no change
    char_count.lhs("test");
    assert_eq!(0, char_count.result().unwrap());

    // With behaviour
    {
        let f = STRING_NUMBER_OPERATIONS.get(&behaviour_ty).cloned().expect("Failed to find behaviour function");
        let factory = StringNumberOperationFactory::new(behaviour_ty, f);
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        char_count.lhs("test");
        assert_eq!(4, char_count.result().unwrap());
        char_count.lhs("test test");
        assert_eq!(9, char_count.result().unwrap());
        char_count.lhs("test");
        assert_eq!(4, char_count.result().unwrap());
    }

    // No behaviour -> no change
    char_count.lhs("test test");
    assert_eq!(4, char_count.result().unwrap());
}
