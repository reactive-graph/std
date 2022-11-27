use crate::model::component_type;
use crate::model::properties;
use crate::NAMESPACE_LOGICAL;

properties!(GeneratorProperties, (TRIGGER, "trigger", false));

component_type!(COMPONENT_GENERATOR, NAMESPACE_LOGICAL, COMPONENT_NAME_GENERATOR, "generator");
