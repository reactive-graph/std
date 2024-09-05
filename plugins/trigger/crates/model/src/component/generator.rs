use crate::NAMESPACE_TRIGGER;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(GeneratorProperties, (TRIGGER, "trigger", false));

component_ty!(COMPONENT_GENERATOR, NAMESPACE_TRIGGER, COMPONENT_NAME_GENERATOR, "generator");
