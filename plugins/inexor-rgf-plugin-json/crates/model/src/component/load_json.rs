use crate::model::behaviour_ty;
use crate::model::component_behaviour_ty;
use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_JSON;

properties!(LoadJsonProperties, (RESULT, "result", {}));

component_ty!(COMPONENT_LOAD_JSON, NAMESPACE_JSON, COMPONENT_NAME_LOAD_JSON, "load_json");
behaviour_ty!(BEHAVIOUR_LOAD_JSON, NAMESPACE_JSON, BEHAVIOUR_NAME_LOAD_JSON, "load_json");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_LOAD_JSON, COMPONENT_LOAD_JSON, BEHAVIOUR_LOAD_JSON);

component_model!(
    ComponentLoadJson,
    get result object
);
