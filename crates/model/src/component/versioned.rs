use crate::model::component_type;
use crate::model::properties;
use crate::NAMESPACE_BASE;

properties!(VersionedProperties, (VERSION, "version", ""));

component_type!(COMPONENT_VERSIONED, NAMESPACE_BASE, COMPONENT_NAME_VERSIONED, "versioned");
