use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_base::Named;
use crate::model_file::File;
use crate::model_result::ResultAny;
use crate::model_trigger::Action;
use crate::ComponentLoadJson;
use crate::NAMESPACE_JSON;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_LOAD_JSON, NAMESPACE_JSON, ENTITY_TYPE_NAME_LOAD_JSON, "load_json");

entity_model!(LoadJson);
impl ComponentLoadJson for LoadJson {}
impl File for LoadJson {}
impl Action for LoadJson {}
impl Named for LoadJson {}
impl ResultAny for LoadJson {}
