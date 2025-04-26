use std::sync::Arc;

use serde_json::json;

use crate::behaviour::entity::operation::LOGICAL_OPERATIONS;
use crate::behaviour::entity::operation::LogicalOperationFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use reactive_graph_graph::BehaviourTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::ReactiveEntityInstance;
use reactive_graph_std_logical_model::COMPONENT_LOGICAL_OPERATION;
use reactive_graph_std_logical_model::LogicalOperation;
use reactive_graph_std_logical_model::LogicalOperationProperties;
use reactive_graph_std_logical_model::NAMESPACE_LOGICAL;
use reactive_graph_std_result_model::ResultBoolean;
use reactive_graph_std_result_model::ResultBooleanProperties;

const LHS: LogicalOperationProperties = LogicalOperationProperties::LHS;
const RESULT: ResultBooleanProperties = ResultBooleanProperties::RESULT;

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
    let reactive_instance = logical_operation(&entity_ty);

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

#[test]
fn rx_not_operation_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_NOT);
    let reactive_instance = logical_operation(&entity_ty);

    let rx_not = LogicalOperation::from(reactive_instance.clone());

    assert_eq!(NAMESPACE_LOGICAL, rx_not.namespace().as_str());
    assert_eq!(TYPE_NAME_NOT, rx_not.type_name().as_str());

    {
        let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_NOT);
        let not_function = LOGICAL_OPERATIONS.get(&behaviour_ty).expect("Failed to get function");
        let not_factory = LogicalOperationFactory::new(behaviour_ty, not_function.clone());
        let behaviour = not_factory.create(reactive_instance.clone());
        assert!(behaviour.is_ok());

        rx_not.lhs(true);
        assert_eq!(false, rx_not.result().unwrap());
        rx_not.lhs(false);
        assert_eq!(true, rx_not.result().unwrap());
    }
    // The behaviour has been dropped (no more changes)
    rx_not.lhs(true);
    assert_eq!(true, rx_not.result().unwrap());
}

fn logical_operation(entity_ty: &EntityTypeId) -> Arc<ReactiveEntityInstance> {
    ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(LHS, json!(false))
        .property(RESULT, json!(false))
        .component(COMPONENT_LOGICAL_OPERATION.clone())
        .build()
}
