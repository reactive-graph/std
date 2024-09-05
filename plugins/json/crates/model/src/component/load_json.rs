use crate::NAMESPACE_JSON;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::component_behaviour_ty;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;

component_ty!(COMPONENT_LOAD_JSON, NAMESPACE_JSON, COMPONENT_NAME_LOAD_JSON, "load_json");
behaviour_ty!(BEHAVIOUR_LOAD_JSON, NAMESPACE_JSON, BEHAVIOUR_NAME_LOAD_JSON, "load_json");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_LOAD_JSON, COMPONENT_LOAD_JSON, BEHAVIOUR_LOAD_JSON);

component_model!(ComponentLoadJson);
