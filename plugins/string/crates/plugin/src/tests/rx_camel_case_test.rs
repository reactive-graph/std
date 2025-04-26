use crate::behaviour::entity::string_operation::STRING_OPERATIONS;
use crate::behaviour::entity::string_operation::StringOperationFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model_string::COMPONENT_STRING_OPERATION;
use crate::model_string::StringOperation;
use crate::model_string::StringOperationProperties::LHS;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use reactive_graph_graph::BehaviourTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_std_result_model::ResultString;
use reactive_graph_std_result_model::ResultStringProperties::RESULT;
use reactive_graph_std_string_model::NAMESPACE_STRING;

#[test]
fn rx_camel_case_test() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_STRING, "camel_case");
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_STRING, "camel_case");

    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty)
        .property_with_default(&LHS)
        .property_with_default(&RESULT)
        .component(COMPONENT_STRING_OPERATION.clone())
        .build();
    let camel_case = StringOperation::from(reactive_instance.clone());

    // No behaviour
    camel_case.lhs("camel_case");
    assert_eq!("", camel_case.result().unwrap());

    // With behaviour
    {
        let f = STRING_OPERATIONS.get(&behaviour_ty).cloned().unwrap();
        let factory = StringOperationFactory::new(behaviour_ty, f);
        let behaviour = factory.create(reactive_instance.clone()).expect("Failed to create behaviour");
        assert_eq!(BehaviourState::Connected, behaviour.get_state());

        camel_case.lhs("camel_case");
        assert_eq!("camelCase", camel_case.result().unwrap());
    }
}
