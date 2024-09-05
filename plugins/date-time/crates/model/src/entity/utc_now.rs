use crate::NAMESPACE_DATE_TIME;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_model_result::ResultString;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;

entity_ty!(ENTITY_TYPE_UTC_NOW, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_UTC_NOW, "utc_now");
behaviour_ty!(BEHAVIOUR_UTC_NOW, NAMESPACE_DATE_TIME, BEHAVIOUR_NAME_UTC_NOW, "utc_now");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_UTC_NOW, ENTITY_TYPE_UTC_NOW, BEHAVIOUR_UTC_NOW);

entity_model!(UtcNow);
impl Action for UtcNow {}
impl ResultString for UtcNow {}
