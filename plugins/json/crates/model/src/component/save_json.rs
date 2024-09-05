use crate::NAMESPACE_JSON;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::component_behaviour_ty;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(SaveJsonProperties, (PAYLOAD, "payload", {}));

component_ty!(COMPONENT_SAVE_JSON, NAMESPACE_JSON, COMPONENT_NAME_SAVE_JSON, "save_json");
behaviour_ty!(BEHAVIOUR_SAVE_JSON, NAMESPACE_JSON, BEHAVIOUR_NAME_SAVE_JSON, "save_json");
component_behaviour_ty!(COMPONENT_BEHAVIOUR_SAVE_JSON, COMPONENT_SAVE_JSON, BEHAVIOUR_SAVE_JSON);

component_model!(
    ComponentSaveJson,
    set payload value
);
