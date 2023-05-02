use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultString;
use crate::model_runtime::Action;
use crate::NAMESPACE_DATE_TIME;

entity_ty!(ENTITY_TYPE_UTC_NOW, NAMESPACE_DATE_TIME, ENTITY_TYPE_NAME_UTC_NOW, "utc_now");
behaviour_ty!(BEHAVIOUR_UTC_NOW, NAMESPACE_DATE_TIME, BEHAVIOUR_NAME_UTC_NOW, "utc_now");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_UTC_NOW, ENTITY_TYPE_UTC_NOW, BEHAVIOUR_UTC_NOW);

entity_model!(UtcNow);
impl Action for UtcNow {}
impl ResultString for UtcNow {}
