use crate::behaviour::entity::string_bool_operation::STRING_BOOL_OPERATIONS;
use crate::behaviour::entity::string_bool_operation::StringBoolOperationFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_string::COMPONENT_STRING_BOOL_OPERATION;
use crate::model_string::StringBoolOperation;
use crate::model_string::StringBoolOperationProperties::LHS;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use reactive_graph_graph::BehaviourTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_std_result_model::ResultBoolean;
use reactive_graph_std_result_model::ResultBooleanProperties::RESULT;
use reactive_graph_std_string_model::NAMESPACE_STRING;

#[test]
fn rx_is_lowercase_test() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_STRING, "is_lowercase");
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_STRING, "is_lowercase");

    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty)
        .property_with_default(&LHS)
        .property_with_default(&RESULT)
        .component(COMPONENT_STRING_BOOL_OPERATION.clone())
        .build();
    let is_lowercase = StringBoolOperation::from(reactive_instance.clone());

    // No behaviour -> no change
    is_lowercase.lhs("test");
    assert!(!is_lowercase.result().unwrap());

    // With behaviour
    {
        let f = STRING_BOOL_OPERATIONS.get(&behaviour_ty).cloned().expect("Failed to find behaviour function");
        let factory = StringBoolOperationFactory::new(behaviour_ty, f);
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        is_lowercase.lhs("test");
        assert!(is_lowercase.result().unwrap());
        is_lowercase.lhs("Test");
        assert!(!is_lowercase.result().unwrap());
        is_lowercase.lhs("test");
        assert!(is_lowercase.result().unwrap());
        is_lowercase.lhs("TEST");
        assert!(!is_lowercase.result().unwrap());
    }

    // No behaviour -> no change
    is_lowercase.lhs("test");
    assert!(!is_lowercase.result().unwrap());
}
