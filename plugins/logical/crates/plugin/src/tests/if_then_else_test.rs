use serde_json::json;

use crate::behaviour::entity::if_then_else::IfThenElseFactory;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::reactive::BehaviourFactory;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_std_logical_model::BEHAVIOUR_IF_THEN_ELSE;
use reactive_graph_std_logical_model::COMPONENT_CONDITION;
use reactive_graph_std_logical_model::Condition;
use reactive_graph_std_logical_model::ConditionProperties::CONDITION;
use reactive_graph_std_logical_model::ENTITY_TYPE_IF_THEN_ELSE;
use reactive_graph_std_logical_model::ENTITY_TYPE_NAME_IF_THEN_ELSE;
use reactive_graph_std_logical_model::IfThenElse;
use reactive_graph_std_logical_model::IfThenElseProperties::ELSE_PAYLOAD;
use reactive_graph_std_logical_model::IfThenElseProperties::THEN_PAYLOAD;
use reactive_graph_std_logical_model::NAMESPACE_LOGICAL;
use reactive_graph_std_result_model::ResultAny;
use reactive_graph_std_result_model::ResultAnyProperties::RESULT;

#[test]
fn if_then_else_test() {
    let entity_ty = ENTITY_TYPE_IF_THEN_ELSE.clone();
    let reactive_instance = ReactiveEntityInstanceBuilder::new(entity_ty.clone())
        .property(CONDITION, json!(false))
        .property(RESULT, json!(false))
        .property(THEN_PAYLOAD, json!("then-payload"))
        .property(ELSE_PAYLOAD, json!("else-payload"))
        .component(COMPONENT_CONDITION.clone())
        .build();

    let if_then_else = IfThenElse::from(reactive_instance.clone());

    assert_eq!(NAMESPACE_LOGICAL, if_then_else.namespace().as_str());
    assert_eq!(ENTITY_TYPE_NAME_IF_THEN_ELSE, if_then_else.type_name().as_str());

    {
        let behaviour_ty = BEHAVIOUR_IF_THEN_ELSE.clone();
        let factory = IfThenElseFactory::new(behaviour_ty.clone());
        let behaviour = factory.create(reactive_instance.clone());
        assert!(behaviour.is_ok());

        if_then_else.condition(true);
        assert_eq!("then-payload", if_then_else.result().unwrap().as_str().unwrap());
        if_then_else.condition(false);
        assert_eq!("else-payload", if_then_else.result().unwrap().as_str().unwrap());
    }
    // The behaviour has been dropped
    if_then_else.condition(true);
    assert_eq!("else-payload", if_then_else.result().unwrap().as_str().unwrap());
}
