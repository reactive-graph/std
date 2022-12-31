use crate::model::component_ty;
use crate::model::properties;
use crate::model::relation_model;
use crate::NAMESPACE_CONNECTOR;

properties!(
    ConnectorProperties,
    (OUTBOUND_PROPERTY_NAME, "outbound_property_name", ""),
    (INBOUND_PROPERTY_NAME, "inbound_property_name", "")
);

component_ty!(COMPONENT_CONNECTOR, NAMESPACE_CONNECTOR, COMPONENT_NAME_CONNECTOR, "connector");

relation_model!(Connector, get outbound_property_name string, get inbound_property_name string);
