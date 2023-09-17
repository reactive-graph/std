use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::component_behaviour_ty;
use crate::NAMESPACE_BINARY;
use inexor_rgf_graph::component_ty;

component_ty!(COMPONENT_SAVE_BINARY_DATA, NAMESPACE_BINARY, COMPONENT_NAME_SAVE_BINARY_DATA, "save_binary_data");
behaviour_ty!(BEHAVIOUR_SAVE_BINARY_DATA, NAMESPACE_BINARY, BEHAVIOUR_NAME_SAVE_BINARY_DATA, "save_binary_data");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_SAVE_BINARY_DATA, COMPONENT_SAVE_BINARY_DATA, BEHAVIOUR_SAVE_BINARY_DATA);
