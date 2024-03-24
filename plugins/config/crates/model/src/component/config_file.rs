use crate::NAMESPACE_CONFIG;
use inexor_rgf_behaviour_model_api::behaviour_ty;
use inexor_rgf_behaviour_model_api::component_behaviour_ty;
use inexor_rgf_graph::component_ty;

// All properties are defined in the component(s)

component_ty!(COMPONENT_CONFIG_FILE, NAMESPACE_CONFIG, COMPONENT_NAME_CONFIG_FILE, "config_file");
behaviour_ty!(BEHAVIOUR_CONFIG_FILE, NAMESPACE_CONFIG, BEHAVIOUR_NAME_CONFIG_FILE, "config_file");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_CONFIG_FILE, COMPONENT_CONFIG_FILE, BEHAVIOUR_CONFIG_FILE);
