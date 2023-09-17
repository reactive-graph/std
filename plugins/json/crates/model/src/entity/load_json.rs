use crate::ComponentLoadJson;
use crate::NAMESPACE_JSON;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_base::Named;
use inexor_rgf_model_file::File;
use inexor_rgf_model_result::ResultAny;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_LOAD_JSON, NAMESPACE_JSON, ENTITY_TYPE_NAME_LOAD_JSON, "load_json");

entity_model!(LoadJson);
impl ComponentLoadJson for LoadJson {}
impl File for LoadJson {}
impl Action for LoadJson {}
impl Named for LoadJson {}
impl ResultAny for LoadJson {}
