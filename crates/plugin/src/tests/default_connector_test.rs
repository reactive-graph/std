use crate::behaviour::relation::connector::Connector;
use crate::behaviour::relation::connector::CONNECTORS;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::tests::utils::create_connector;
use crate::tests::utils::create_random_entity_instance;
use crate::tests::utils::r_string;
use serde_json::json;
use std::sync::Arc;

pub static TYPE_NAME_DEFAULT_CONNECTOR: &str = "default_connector";

// Necessary because the type_name of an edge in the graph database between two vertexes must
// be unique.
#[test]
fn test_type_name_construction() {
    let outbound_property_name = "lhs";
    let inbound_property_name = "result";
    let full_type_name = Connector::type_name(TYPE_NAME_DEFAULT_CONNECTOR, outbound_property_name, inbound_property_name);
    assert_eq!("default_connector--lhs--result", full_type_name);
}

#[test]
fn default_connector_test() {
    let namespace = r_string();
    let outbound_property_name = r_string();
    let inbound_property_name = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(outbound_property_name.clone()));
    let inbound_entity = Arc::new(create_random_entity_instance(inbound_property_name.clone()));
    let r = Arc::new(create_connector(
        namespace.as_str(),
        outbound_entity.clone(),
        TYPE_NAME_DEFAULT_CONNECTOR,
        inbound_entity.clone(),
        outbound_property_name.as_str(),
        inbound_property_name.as_str(),
    ));
    let propagation_function = CONNECTORS.get(TYPE_NAME_DEFAULT_CONNECTOR);
    let mut connector = Connector::from_relation(r.clone(), *propagation_function.unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(true));
    assert!(connector.relation.inbound.as_bool(inbound_property_name.clone()).unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(false));
    assert!(!connector.relation.inbound.as_bool(inbound_property_name.clone()).unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(123));
    assert_eq!(123, connector.relation.inbound.as_u64(inbound_property_name.clone()).unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(-123));
    assert_eq!(-123, connector.relation.inbound.as_i64(inbound_property_name.clone()).unwrap());
    connector.relation.outbound.set(outbound_property_name.clone(), json!(1.23));
    assert_eq!(1.23, connector.relation.inbound.as_f64(inbound_property_name.clone()).unwrap());
    let s = r_string();
    connector.relation.outbound.set(outbound_property_name.clone(), json!(s.clone()));
    assert_eq!(s, connector.relation.inbound.as_string(inbound_property_name.clone()).unwrap());
    // Disconnect, no more propagation
    connector.disconnect();
    connector
        .relation
        .outbound
        .set(outbound_property_name.clone(), json!("MUST NOT PROPAGATED ANYMORE"));
    assert_eq!(s, connector.relation.inbound.as_string(inbound_property_name.clone()).unwrap());
    // Reconnect, should propagate again
    connector.connect();
    let s2 = r_string();
    connector.relation.outbound.set(outbound_property_name.clone(), json!(s2.clone()));
    assert_eq!(s2, connector.relation.inbound.as_string(inbound_property_name.clone()).unwrap());
}
