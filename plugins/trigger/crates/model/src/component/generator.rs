use crate::NAMESPACE_TRIGGER;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(GeneratorProperties, (TRIGGER, "trigger", false));

component_ty!(COMPONENT_GENERATOR, NAMESPACE_TRIGGER, COMPONENT_NAME_GENERATOR, "generator");
