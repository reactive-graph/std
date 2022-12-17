use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_LOGICAL;

properties!(GeneratorProperties, (TRIGGER, "trigger", false));

component_ty!(COMPONENT_GENERATOR, NAMESPACE_LOGICAL, COMPONENT_NAME_GENERATOR, "generator");
