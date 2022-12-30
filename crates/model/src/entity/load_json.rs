use crate::model::entity_model;
use crate::model::entity_ty;
use crate::Action;
use crate::ComponentLoadJson;
use crate::File;
use crate::NAMESPACE_JSON;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_LOAD_JSON, NAMESPACE_JSON, ENTITY_TYPE_NAME_LOAD_JSON, "load_json");

entity_model!(
    LoadJson,
    // get result value,
    // set trigger bool,
    // set filename string,
);
impl ComponentLoadJson for LoadJson {}
impl File for LoadJson {}
impl Action for LoadJson {}
// impl model_base::Named for LoadJson {}
