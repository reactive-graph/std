use std::sync::Arc;

use serde_json::json;

use crate::behaviour::entity::gate::LOGICAL_GATES;
use crate::behaviour::entity::gate::LogicalGateFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use reactive_graph_graph::BehaviourTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::ReactiveEntityInstance;
use reactive_graph_std_logical_model::COMPONENT_LOGICAL_GATE;
use reactive_graph_std_logical_model::LogicalGate;
use reactive_graph_std_logical_model::LogicalGateProperties;
use reactive_graph_std_logical_model::NAMESPACE_LOGICAL;
use reactive_graph_std_result_model::ResultBoolean;
use reactive_graph_std_result_model::ResultBooleanProperties;

const LHS: LogicalGateProperties = LogicalGateProperties::LHS;
const RHS: LogicalGateProperties = LogicalGateProperties::RHS;
const RESULT: ResultBooleanProperties = ResultBooleanProperties::RESULT;

const TYPE_NAME_AND: &str = "and";

#[test]
fn logical_gate_behaviour_function_should_exist() {
    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_AND);
    assert!(LOGICAL_GATES.contains_key(&behaviour_ty));
    assert!(LOGICAL_GATES.get(&behaviour_ty).is_some());
}

#[test]
fn and_gate_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_AND);
    let reactive_instance = logical_gate(&entity_ty);

    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_AND);
    let and_function = LOGICAL_GATES.get(&behaviour_ty).expect("Failed to get function");
    let and_factory = LogicalGateFactory::new(behaviour_ty, and_function.clone());
    let behaviour = and_factory.create(reactive_instance.clone()).expect("Failed to create behaviour");

    assert_eq!(NAMESPACE_LOGICAL, behaviour.ty().namespace().as_str());
    assert_eq!(TYPE_NAME_AND, behaviour.ty().type_name().as_str());

    // === Reactive Entity API ===

    reactive_instance.set(LHS, json!(true));
    reactive_instance.set(RHS, json!(true));
    // Expect the correct output value -> behaviour has modified the output
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
    reactive_instance.set(LHS, json!(false));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());
    reactive_instance.set(RHS, json!(false));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());
    reactive_instance.set(LHS, json!(true));
    assert_eq!(false, reactive_instance.as_bool(RESULT).unwrap());
    reactive_instance.set(RHS, json!(true));
    assert_eq!(true, reactive_instance.as_bool(RESULT).unwrap());
}

#[test]
fn incomplete_and_gate_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_AND);
    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .component(COMPONENT_LOGICAL_GATE.clone())
        .build();

    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_AND);
    let and_function = LOGICAL_GATES.get(&behaviour_ty).expect("Failed to get function");
    let and_factory = LogicalGateFactory::new(behaviour_ty, and_function.clone());
    let behaviour = and_factory.create(reactive_instance.clone());
    assert!(behaviour.is_err());
}

#[test]
fn rx_and_gate_test() {
    let entity_ty = EntityTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_AND);
    let reactive_instance = logical_gate(&entity_ty);

    let rx_and = LogicalGate::from(reactive_instance.clone());

    assert_eq!(NAMESPACE_LOGICAL, rx_and.namespace().as_str());
    assert_eq!(TYPE_NAME_AND, rx_and.type_name().as_str());

    {
        let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_LOGICAL, TYPE_NAME_AND);
        let and_function = LOGICAL_GATES.get(&behaviour_ty).expect("Failed to get function");
        let and_factory = LogicalGateFactory::new(behaviour_ty, and_function.clone());
        let behaviour = and_factory.create(reactive_instance.clone());
        assert!(behaviour.is_ok());

        rx_and.lhs(true);
        rx_and.rhs(true);
        assert_eq!(true, rx_and.result().unwrap());
        rx_and.lhs(false);
        assert_eq!(false, rx_and.result().unwrap());
        rx_and.rhs(false);
        assert_eq!(false, rx_and.result().unwrap());
        rx_and.lhs(true);
        assert_eq!(false, rx_and.result().unwrap());
        rx_and.rhs(true);
        assert_eq!(true, rx_and.result().unwrap());
    }
    // The behaviour has been dropped (no more changes)
    rx_and.lhs(false);
    assert_eq!(true, rx_and.result().unwrap());
}

fn logical_gate(entity_ty: &EntityTypeId) -> Arc<ReactiveEntityInstance> {
    ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(LHS, json!(false))
        .property(RHS, json!(false))
        .property(RESULT, json!(false))
        .component(COMPONENT_LOGICAL_GATE.clone())
        .build()
}
