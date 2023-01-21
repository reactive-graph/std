use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultNumberU64;
use crate::model_trigger::Action;
use crate::NAMESPACE_DATE_TIME;

entity_ty!(ENTITY_TYPE_UTC_TIMESTAMP, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_UTC_TIMESTAMP, "utc_timestamp");
behaviour_ty!(BEHAVIOUR_UTC_TIMESTAMP, NAMESPACE_DATE_TIME, BEHAVIOUR_NAME_UTC_TIMESTAMP, "utc_timestamp");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_UTC_TIMESTAMP, ENTITY_TYPE_UTC_TIMESTAMP, BEHAVIOUR_UTC_TIMESTAMP);

entity_model!(UtcTimestamp);
impl Action for UtcTimestamp {}
impl ResultNumberU64 for UtcTimestamp {}
