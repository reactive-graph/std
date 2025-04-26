use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;
use reactive_graph_reactive_model_api::entity_model;
use serde_json::json;

use reactive_graph_std_result_model::ResultString;

use crate::NAMESPACE_STRING;

properties!(TemplatingProperties, (TEMPLATE, "template", ""), (CONTEXT, "context", json!({})));

entity_ty!(ENTITY_TYPE_TEMPLATING, NAMESPACE_STRING, ENTITY_TYPE_NAME_TEMPLATING, "templating");
behaviour_ty!(BEHAVIOUR_TEMPLATING, NAMESPACE_STRING, BEHAVIOUR_NAME_TEMPLATING, "templating");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_TEMPLATING, ENTITY_TYPE_TEMPLATING, BEHAVIOUR_TEMPLATING);

entity_model!(
    Templating,
    set context value,
    set template string
);
impl ResultString for Templating {}
