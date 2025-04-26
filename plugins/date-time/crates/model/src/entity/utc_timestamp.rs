use crate::NAMESPACE_DATE_TIME;
use reactive_graph_behaviour_model_api::behaviour_ty;
use reactive_graph_behaviour_model_api::entity_behaviour_ty;
use reactive_graph_graph::entity_ty;
use reactive_graph_reactive_model_api::entity_model;
use reactive_graph_runtime_model::Action;
use reactive_graph_std_result_model::ResultNumberU64;

entity_ty!(ENTITY_TYPE_UTC_TIMESTAMP, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_UTC_TIMESTAMP, "utc_timestamp");
behaviour_ty!(BEHAVIOUR_UTC_TIMESTAMP, NAMESPACE_DATE_TIME, BEHAVIOUR_NAME_UTC_TIMESTAMP, "utc_timestamp");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_UTC_TIMESTAMP, ENTITY_TYPE_UTC_TIMESTAMP, BEHAVIOUR_UTC_TIMESTAMP);

entity_model!(UtcTimestamp);
impl Action for UtcTimestamp {}
impl ResultNumberU64 for UtcTimestamp {}
