use crate::NAMESPACE_BASE;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(NamedProperties, (NAME, "name", ""));

component_ty!(COMPONENT_NAMED, NAMESPACE_BASE, COMPONENT_NAME_NAMED, "named");

component_model!(
    Named,
    data name string,
);
