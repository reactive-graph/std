use crate::behaviour::entity::string_operation::StringOperationFactory;
use crate::behaviour::entity::string_operation::STRING_OPERATIONS;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_string::StringOperation;
use crate::model_string::StringOperationProperties::LHS;
use crate::model_string::COMPONENT_STRING_OPERATION;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use reactive_graph_graph::BehaviourTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_model_result::ResultString;
use reactive_graph_model_result::ResultStringProperties::RESULT;
use reactive_graph_model_string::NAMESPACE_STRING;

#[test]
fn rx_pascal_case_test() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_STRING, "pascal_case");
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_STRING, "pascal_case");

    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty)
        .property_with_default(&LHS)
        .property_with_default(&RESULT)
        .component(COMPONENT_STRING_OPERATION.clone())
        .build();
    let pascal_case = StringOperation::from(reactive_instance.clone());

    // No behaviour
    pascal_case.lhs("pascal_case");
    assert_eq!("", pascal_case.result().unwrap());

    // With behaviour
    {
        let f = STRING_OPERATIONS.get(&behaviour_ty).cloned().unwrap();
        let factory = StringOperationFactory::new(behaviour_ty, f);
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        pascal_case.lhs("pascal_case");
        assert_eq!("PascalCase", pascal_case.result().unwrap());
    }
}
