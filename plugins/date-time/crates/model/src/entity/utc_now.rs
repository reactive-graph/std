use crate::behaviour_api::behaviour_ty;
use crate::behaviour_api::entity_behaviour_ty;
use crate::NAMESPACE_DATE_TIME;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_model_result::ResultString;
use inexor_rgf_model_runtime::Action;
use inexor_rgf_reactive_api::entity_model;

entity_ty!(ENTITY_TYPE_UTC_NOW, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_UTC_NOW, "utc_now");
behaviour_ty!(BEHAVIOUR_UTC_NOW, NAMESPACE_DATE_TIME, BEHAVIOUR_NAME_UTC_NOW, "utc_now");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_UTC_NOW, ENTITY_TYPE_UTC_NOW, BEHAVIOUR_UTC_NOW);

entity_model!(UtcNow);
impl Action for UtcNow {}
impl ResultString for UtcNow {}
