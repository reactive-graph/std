use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_BASE;

properties!(VersionedProperties, (VERSION, "version", ""));

component_ty!(COMPONENT_VERSIONED, NAMESPACE_BASE, COMPONENT_NAME_VERSIONED, "versioned");

component_model!(
    Versioned,
    data version string,
);
