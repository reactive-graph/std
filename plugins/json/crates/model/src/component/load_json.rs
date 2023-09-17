use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::component_behaviour_ty;
use crate::NAMESPACE_JSON;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;

component_ty!(COMPONENT_LOAD_JSON, NAMESPACE_JSON, COMPONENT_NAME_LOAD_JSON, "load_json");
behaviour_ty!(BEHAVIOUR_LOAD_JSON, NAMESPACE_JSON, BEHAVIOUR_NAME_LOAD_JSON, "load_json");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_LOAD_JSON, COMPONENT_LOAD_JSON, BEHAVIOUR_LOAD_JSON);

component_model!(ComponentLoadJson);
