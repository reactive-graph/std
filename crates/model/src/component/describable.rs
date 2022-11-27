use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_BASE;

properties!(DescribableProperties, (DESCRIPTION, "description", ""));

component_ty!(COMPONENT_DESCRIBABLE, NAMESPACE_BASE, COMPONENT_NAME_DESCRIBABLE, "describable");
