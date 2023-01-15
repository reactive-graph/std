use crate::model::behaviour_ty;
use crate::model::component_behaviour_ty;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_JSON;
use inexor_rgf_core_model::component_model;

properties!(SaveJsonProperties, (PAYLOAD, "payload", {}));

component_ty!(COMPONENT_SAVE_JSON, NAMESPACE_JSON, COMPONENT_NAME_SAVE_JSON, "save_json");
behaviour_ty!(BEHAVIOUR_SAVE_JSON, NAMESPACE_JSON, BEHAVIOUR_NAME_SAVE_JSON, "save_json");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_SAVE_JSON, COMPONENT_SAVE_JSON, BEHAVIOUR_SAVE_JSON);

component_model!(
    ComponentSaveJson,
    set payload value
);
