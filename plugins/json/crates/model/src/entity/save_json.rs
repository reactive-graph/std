use crate::ComponentSaveJson;
use crate::NAMESPACE_JSON;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_base::Named;
use inexor_rgf_model_file::File;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_SAVE_JSON, NAMESPACE_JSON, ENTITY_TYPE_NAME_SAVE_JSON, "save_json");

entity_model!(
    SaveJson,
    set payload value,
);
impl ComponentSaveJson for SaveJson {}
impl File for SaveJson {}
impl Named for SaveJson {}
impl Action for SaveJson {}
