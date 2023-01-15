use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_file::File;
use crate::ComponentSaveJson;
use crate::NAMESPACE_JSON;

// All properties are defined in the component(s)

entity_ty!(ENTITY_TYPE_SAVE_JSON, NAMESPACE_JSON, ENTITY_TYPE_NAME_SAVE_JSON, "save_json");

entity_model!(
    SaveJson,
    get result value,
    set payload value,
    set trigger bool,
);
impl ComponentSaveJson for SaveJson {}
impl File for SaveJson {}
