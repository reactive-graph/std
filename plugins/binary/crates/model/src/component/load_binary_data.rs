use crate::NAMESPACE_BINARY;
use inexor_rgf_behaviour_model_api::behaviour_ty;
use inexor_rgf_behaviour_model_api::component_behaviour_ty;
use inexor_rgf_graph::component_ty;

component_ty!(COMPONENT_LOAD_BINARY_DATA, NAMESPACE_BINARY, COMPONENT_NAME_LOAD_BINARY_DATA, "load_binary_data");
behaviour_ty!(BEHAVIOUR_LOAD_BINARY_DATA, NAMESPACE_BINARY, BEHAVIOUR_NAME_LOAD_BINARY_DATA, "load_binary_data");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_LOAD_BINARY_DATA, COMPONENT_LOAD_BINARY_DATA, BEHAVIOUR_LOAD_BINARY_DATA);
