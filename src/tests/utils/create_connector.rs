use crate::behaviour::relation::connector::Connector;
use crate::model::{ReactiveEntityInstance, ReactiveRelationInstance};
use crate::tests::utils::create_connector_relation_instance_with_properties;
use std::sync::Arc;

pub fn create_connector<S: Into<String>>(
    outbound_entity: Arc<ReactiveEntityInstance>,
    type_name: S,
    inbound_entity: Arc<ReactiveEntityInstance>,
    outbound_property_name: S,
    inbound_property_name: S,
) -> ReactiveRelationInstance {
    let outbound_property_name = outbound_property_name.into();
    let inbound_property_name = inbound_property_name.into();
    let type_name = Connector::type_name(type_name.into(), outbound_property_name.clone(), inbound_property_name.clone());
    create_connector_relation_instance_with_properties(
        outbound_entity,
        type_name.into(),
        inbound_entity,
        outbound_property_name.clone(),
        inbound_property_name.clone(),
    )
}
