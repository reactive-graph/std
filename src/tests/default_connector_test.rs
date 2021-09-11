use crate::behaviour::relation::connector::Connector;
use crate::behaviour::relation::connector::CONNECTORS;
use crate::model::{PropertyInstanceGetter, PropertyInstanceSetter};
use crate::tests::utils::create_connector;
use crate::tests::utils::create_random_entity_instance;
use crate::tests::utils::r_string;
use serde_json::json;
use std::sync::Arc;

#[test]
fn propagation_function_test() {
    let expected_propagation_functions = vec![
        "debug_connector",
        "default_connector",
        "parse_float_connector",
        "parse_int_connector",
        "to_string_connector",
        "trace_connector",
    ];
    assert_eq!(
        expected_propagation_functions.len(),
        CONNECTORS
            .keys()
            .into_iter()
            .filter(|function_name| expected_propagation_functions.contains(function_name))
            .count()
    );
}

// Necessary because the type_name of an edge in the graph database between two vertexes must
// be unique.
#[test]
fn test_type_name_construction() {
    let type_name = "default_connector";
    let outbound_property_name = "lhs";
    let inbound_property_name = "result";
    let full_type_name =
        Connector::type_name(type_name, outbound_property_name, inbound_property_name);
    assert_eq!("default_connector--lhs--result", full_type_name);
}

#[test]
fn default_connector_test() {
    let outbound_property_name = r_string();
    let inbound_property_name = r_string();
    let outbound_entity = Arc::new(create_random_entity_instance(
        outbound_property_name.clone(),
    ));
    let inbound_entity = Arc::new(create_random_entity_instance(inbound_property_name.clone()));
    let type_name = "default_connector";
    let r = Arc::new(create_connector(
        outbound_entity.clone(),
        type_name,
        inbound_entity.clone(),
        outbound_property_name.as_str(),
        inbound_property_name.as_str(),
    ));
    let propagation_function = CONNECTORS.get(type_name);
    let mut connector = Connector::from_relation(r.clone(), *propagation_function.unwrap());
    connector
        .relation
        .outbound
        .set(outbound_property_name.clone(), json!(true));
    assert!(connector
        .relation
        .inbound
        .as_bool(inbound_property_name.clone())
        .unwrap());
    connector
        .relation
        .outbound
        .set(outbound_property_name.clone(), json!(false));
    assert!(!connector
        .relation
        .inbound
        .as_bool(inbound_property_name.clone())
        .unwrap());
    connector
        .relation
        .outbound
        .set(outbound_property_name.clone(), json!(123));
    assert_eq!(
        123,
        connector
            .relation
            .inbound
            .as_u64(inbound_property_name.clone())
            .unwrap()
    );
    connector
        .relation
        .outbound
        .set(outbound_property_name.clone(), json!(-123));
    assert_eq!(
        -123,
        connector
            .relation
            .inbound
            .as_i64(inbound_property_name.clone())
            .unwrap()
    );
    connector
        .relation
        .outbound
        .set(outbound_property_name.clone(), json!(1.23));
    assert_eq!(
        1.23,
        connector
            .relation
            .inbound
            .as_f64(inbound_property_name.clone())
            .unwrap()
    );
    let s = r_string();
    connector
        .relation
        .outbound
        .set(outbound_property_name.clone(), json!(s.clone()));
    assert_eq!(
        s,
        connector
            .relation
            .inbound
            .as_string(inbound_property_name.clone())
            .unwrap()
    );
    // Disconnect, no more propagation
    connector.disconnect();
    connector.relation.outbound.set(
        outbound_property_name.clone(),
        json!("MUST NOT PROPAGATED ANYMORE"),
    );
    assert_eq!(
        s,
        connector
            .relation
            .inbound
            .as_string(inbound_property_name.clone())
            .unwrap()
    );
    // Reconnect, should propagate again
    connector.connect();
    let s2 = r_string();
    connector
        .relation
        .outbound
        .set(outbound_property_name.clone(), json!(s2.clone()));
    assert_eq!(
        s2,
        connector
            .relation
            .inbound
            .as_string(inbound_property_name.clone())
            .unwrap()
    );
}
