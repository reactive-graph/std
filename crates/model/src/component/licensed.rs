use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_BASE;

properties!(LicensedProperties, (LICENSE, "license", ""), (ATTRIBUTION, "attribution", ""));

component_ty!(COMPONENT_LICENSED, NAMESPACE_BASE, COMPONENT_NAME_LICENSED, "licensed");

component_model!(
    Licensed,
    data license string,
    data attribution string,
);
