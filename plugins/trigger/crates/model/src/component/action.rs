use crate::NAMESPACE_TRIGGER;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(ActionProperties, (TRIGGER, "trigger", false));

component_ty!(COMPONENT_ACTION, NAMESPACE_TRIGGER, COMPONENT_NAME_ACTION, "action");

component_model!(Action, trigger);
