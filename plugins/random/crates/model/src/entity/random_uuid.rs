use crate::model::behaviour_ty;
use crate::model::entity_behaviour_ty;
use crate::model::entity_model;
use crate::model::entity_ty;
use crate::model_result::ResultString;
use crate::model_runtime::Action;
use crate::NAMESPACE_RANDOM;

entity_ty!(ENTITY_TYPE_RANDOM_UUID, NAMESPACE_RANDOM, ENTITY_TYPE_NAME_RANDOM_UUID, "random_uuid");
behaviour_ty!(BEHAVIOUR_RANDOM_UUID, NAMESPACE_RANDOM, BEHAVIOUR_NAME_RANDOM_UUID, "random_uuid");
entity_behaviour_ty!(ENTITY_BEHAVIOUR_RANDOM_UUID, ENTITY_TYPE_RANDOM_UUID, BEHAVIOUR_RANDOM_UUID);

entity_model!(RandomUuid);
impl Action for RandomUuid {}
impl ResultString for RandomUuid {}
