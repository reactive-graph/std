use serde_json::json;

use crate::behaviour::entity::string_operation::StringOperationFactory;
use crate::behaviour::entity::string_operation::STRING_OPERATIONS;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::BehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model_string::StringOperation;
use crate::model_string::StringOperationProperties::LHS;
use crate::model_string::StringOperationProperties::RESULT;
use crate::model_string::NAMESPACE_STRING;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;

#[test]
fn rx_pascal_case_test() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_STRING, "pascal_case");
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_STRING, "pascal_case");

    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty)
        .property(LHS, json!(""))
        .property(RESULT, json!(""))
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
