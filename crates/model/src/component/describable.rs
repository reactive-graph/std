use crate::model::component_type;
use crate::model::properties;
use crate::NAMESPACE_BASE;

properties!(DescribableProperties, (DESCRIPTION, "description", ""));

component_type!(COMPONENT_DESCRIBABLE, NAMESPACE_BASE, COMPONENT_NAME_DESCRIBABLE, "describable");
