use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_base::Named;
use crate::model_file::File;
use crate::model_trigger::Action;
use crate::ComponentSaveJson;
use crate::NAMESPACE_JSON;

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
