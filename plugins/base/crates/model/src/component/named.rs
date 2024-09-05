use crate::NAMESPACE_BASE;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(NamedProperties, (NAME, "name", ""));

component_ty!(COMPONENT_NAMED, NAMESPACE_BASE, COMPONENT_NAME_NAMED, "named");

component_model!(
    Named,
    data name string,
);
