use crate::NAMESPACE_HTTP;
use inexor_rgf_behaviour_model_api::behaviour_ty;
use inexor_rgf_behaviour_model_api::component_behaviour_ty;
use inexor_rgf_graph::component_ty;

component_ty!(COMPONENT_HTTP, NAMESPACE_HTTP, COMPONENT_NAME_HTTP, "http");
behaviour_ty!(BEHAVIOUR_HTTP, NAMESPACE_HTTP, BEHAVIOUR_NAME_HTTP, "http");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_HTTP, COMPONENT_HTTP, BEHAVIOUR_HTTP);
