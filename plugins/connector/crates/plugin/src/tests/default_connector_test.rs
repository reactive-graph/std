use serde_json::json;

use crate::behaviour::relation::connector::ConnectorFactory;
use crate::behaviour::relation::connector::CONNECTOR_BEHAVIOURS;
use crate::reactive::BehaviourFactory;
use crate::reactive::BehaviourState;
use crate::tests::utils::create_connector;
use crate::tests::utils::create_random_entity_instance;
use crate::tests::utils::r_string;
use inexor_rgf_graph::BehaviourTypeId;
use inexor_rgf_graph::PropertyInstanceGetter;
use inexor_rgf_graph::PropertyInstanceSetter;
use inexor_rgf_graph::ReactivePropertyContainer;
use inexor_rgf_graph::RelationTypeId;
use inexor_rgf_model_connector::NAMESPACE_CONNECTOR;

pub static TYPE_NAME_DEFAULT_CONNECTOR: &str = "default_connector";

#[test]
fn default_connector_test() {
    let ty = RelationTypeId::new_from_type(NAMESPACE_CONNECTOR, TYPE_NAME_DEFAULT_CONNECTOR);

    let outbound_property_name = String::from("ob"); //r_string();
    let outbound_entity = create_random_entity_instance(outbound_property_name.clone());
    assert!(outbound_entity.has_property(&outbound_property_name));

    let inbound_property_name = String::from("ib"); //r_string();
    let inbound_entity = create_random_entity_instance(inbound_property_name.clone());
    assert!(inbound_entity.has_property(&inbound_property_name));

    let relation = create_connector(
        outbound_entity.clone(),
        ty,
        inbound_entity.clone(),
        outbound_property_name.as_str(),
        inbound_property_name.as_str(),
    );
    assert!(relation.outbound.has_property(&outbound_property_name));
    assert!(relation.inbound.has_property(&inbound_property_name));

    let behaviour_ty = BehaviourTypeId::new_from_type(NAMESPACE_CONNECTOR, TYPE_NAME_DEFAULT_CONNECTOR);
    let propagation_function = CONNECTOR_BEHAVIOURS.get(&behaviour_ty);
    let connector_factory = ConnectorFactory::new(behaviour_ty, *propagation_function.expect("Failed to find propagation function"));
    let connector = connector_factory.create(relation.clone()).expect("Failed to create connector");
    // connector.transition(BehaviourState::Connected).expect("Failed to connect");
    let relation = connector.get_reactive_instance();
    relation.outbound.set(outbound_property_name.clone(), json!(true));
    assert!(relation.inbound.as_bool(inbound_property_name.clone()).unwrap());
    relation.outbound.set(outbound_property_name.clone(), json!(false));
    assert!(!relation.inbound.as_bool(inbound_property_name.clone()).unwrap());
    relation.outbound.set(outbound_property_name.clone(), json!(123));
    assert_eq!(123, relation.inbound.as_u64(inbound_property_name.clone()).unwrap());
    relation.outbound.set(outbound_property_name.clone(), json!(-123));
    assert_eq!(-123, relation.inbound.as_i64(inbound_property_name.clone()).unwrap());
    relation.outbound.set(outbound_property_name.clone(), json!(1.23));
    assert_eq!(1.23, relation.inbound.as_f64(inbound_property_name.clone()).unwrap());
    let s = r_string();
    relation.outbound.set(outbound_property_name.clone(), json!(s.clone()));
    assert_eq!(s, relation.inbound.as_string(inbound_property_name.clone()).unwrap());
    // Disconnect, no more propagation
    connector.transition(BehaviourState::Ready).expect("Failed to disconnect");
    relation.outbound.set(outbound_property_name.clone(), json!("MUST NOT PROPAGATED ANYMORE"));
    assert_eq!(s, relation.inbound.as_string(inbound_property_name.clone()).unwrap());
    // Reconnect, should propagate again
    connector.transition(BehaviourState::Connected).expect("Failed to connect again");
    let s2 = r_string();
    relation.outbound.set(outbound_property_name.clone(), json!(s2.clone()));
    assert_eq!(s2, relation.inbound.as_string(inbound_property_name.clone()).unwrap());
}
