use serde_json::json;

use crate::behaviour::entity::operation::LogicalOperationFactory;
use crate::behaviour::entity::operation::LOGICAL_OPERATIONS;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::model::BehaviourTypeId;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model_logical::LogicalOperationProperties;
use crate::model_logical::COMPONENT_LOGICAL_OPERATION;
use crate::model_logical::NAMESPACE_LOGICAL;
use crate::reactive::BehaviourFactory;

const LHS: LogicalOperationProperties = LogicalOperationProperties::LHS;
const RESULT: LogicalOperationProperties = LogicalOperationProperties::RESULT;

const TYPE_NAME_NOT: &str = "not";

#[test]
fn logical_operation_behaviour_function_should_exist() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_NOT);
    assert!(LOGICAL_OPERATIONS.contains_key(&behaviour_ty));
    assert!(LOGICAL_OPERATIONS.get(&behaviour_ty).is_some());
}

#[test]
fn not_operation_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_NOT);
    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(LHS, json!(false))
        .property(RESULT, json!(false))
        .component(COMPONENT_LOGICAL_OPERATION.clone())
        .build();

    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_NOT);
    let not_function = LOGICAL_OPERATIONS.get(&behaviour_ty).expect("Failed to get function");
    let not_factory = LogicalOperationFactory::new(behaviour_ty, not_function.clone());
    let behaviour = not_factory.create(reactive_instance.clone()).expect("Failed to create behaviour");

    assert_eq!(NAMESPACE_LOGICAL, behaviour.ty().namespace().as_str());
    assert_eq!(TYPE_NAME_NOT, behaviour.ty().type_name().as_str());

    // === Reactive Entity API ===

    reactive_instance.set(LHS, json!(true));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());
    reactive_instance.set(LHS, json!(false));
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
}

#[test]
fn incomplete_not_operation_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_NOT);
    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        // Missing properties
        // .property(LHS, json!(false))
        // .property(RESULT, json!(false))
        .component(COMPONENT_LOGICAL_OPERATION.clone())
        .build();

    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_NOT);
    let not_function = LOGICAL_OPERATIONS.get(&behaviour_ty).expect("Failed to get function");
    let not_factory = LogicalOperationFactory::new(behaviour_ty, not_function.clone());
    let behaviour = not_factory.create(reactive_instance.clone());
    assert!(behaviour.is_err());
}
