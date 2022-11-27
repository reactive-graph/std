use crate::model::component_type;
use crate::model::properties;
use crate::NAMESPACE_BASE;

properties!(NamedProperties, (NAME, "name", ""));

component_type!(COMPONENT_NAMED, NAMESPACE_BASE, COMPONENT_NAM_ENAMED, "named");
