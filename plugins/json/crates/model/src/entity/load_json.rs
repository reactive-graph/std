use crate::ComponentLoadJson;
use crate::NAMESPACE_JSON;
use reactive_graph_graph::entity_ty;
use reactive_graph_model_base::Named;
use reactive_graph_model_file::File;
use reactive_graph_model_result::ResultAny;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_LOAD_JSON, NAMESPACE_JSON, ENTITY_TYPE_NAME_LOAD_JSON, "load_json");

entity_model!(LoadJson);
impl ComponentLoadJson for LoadJson {}
impl File for LoadJson {}
impl Action for LoadJson {}
impl Named for LoadJson {}
impl ResultAny for LoadJson {}
