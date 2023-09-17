use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::NAMESPACE_DATE_TIME;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultNumberU64;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

entity_ty!(ENTITY_TYPE_UTC_TIMESTAMP, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_UTC_TIMESTAMP, "utc_timestamp");
behaviour_ty!(BEHAVIOUR_UTC_TIMESTAMP, NAMESPACE_DATE_TIME, BEHAVIOUR_NAME_UTC_TIMESTAMP, "utc_timestamp");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_UTC_TIMESTAMP, ENTITY_TYPE_UTC_TIMESTAMP, BEHAVIOUR_UTC_TIMESTAMP);

entity_model!(UtcTimestamp);
impl Action for UtcTimestamp {}
impl ResultNumberU64 for UtcTimestamp {}
