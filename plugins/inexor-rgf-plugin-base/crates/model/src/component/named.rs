use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_BASE;

properties!(NamedProperties, (NAME, "name", ""));

component_ty!(COMPONENT_NAMED, NAMESPACE_BASE, COMPONENT_NAME_NAMED, "named");

component_model!(
    Named,
    data name string,
);
