use crate::NAMESPACE_TRIGGER;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(ActionProperties, (TRIGGER, "trigger", false));

component_ty!(COMPONENT_ACTION, NAMESPACE_TRIGGER, COMPONENT_NAME_ACTION, "action");

component_model!(Action, trigger);
